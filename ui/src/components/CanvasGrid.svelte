<script lang="ts">
  import { onMount } from 'svelte';
  import { editorState } from '../lib/state.svelte';
  import type { ToolType } from '../lib/types';
  import Tooltip from './Tooltip.svelte';

  let canvasEl: HTMLCanvasElement | null = null;
  let containerEl: HTMLDivElement | null = null;

  let isPanning = $state(false);
  let startPanX = $state(0);
  let startPanY = $state(0);
  let hoverGameX = $state(0);
  let hoverGameY = $state(0);
  let hoverPixelI = $state(0);
  let hoverPixelJ = $state(0);

  const tools: { id: ToolType; label: string; shortcut: string; tip: string }[] = [
    { id: 'select', label: 'V', shortcut: 'V', tip: 'Select and transform layers' },
    { id: 'direct', label: 'A', shortcut: 'A', tip: 'Direct select arms and anchors' },
    { id: 'lines', label: 'L', shortcut: 'L', tip: 'Line arms reticle shape' },
    { id: 'dot', label: 'D', shortcut: 'D', tip: 'Center reticle dot' },
    { id: 'rect', label: 'R', shortcut: 'R', tip: 'Rectangle / Box element' },
    { id: 'circle', label: 'C', shortcut: 'C', tip: 'Circle / Ring element' },
    { id: 'pen', label: 'P', shortcut: 'P', tip: 'Custom vector path' },
    { id: 'brush', label: 'B', shortcut: 'B', tip: 'Pixel brush bitmap editor' },
  ];

  function renderCanvas() {
    if (!canvasEl || !containerEl) return;
    const ctx = canvasEl.getContext('2d');
    if (!ctx) return;

    const dpr = window.devicePixelRatio || 1;
    const width = containerEl.clientWidth;
    const height = containerEl.clientHeight;

    canvasEl.width = width * dpr;
    canvasEl.height = height * dpr;
    canvasEl.style.width = `${width}px`;
    canvasEl.style.height = `${height}px`;

    ctx.save();
    ctx.scale(dpr, dpr);

    // 1. Background (§6.8)
    const bg = editorState.previewBackground;
    if (bg === 'light') {
      ctx.fillStyle = '#F1F5F9';
    } else if (bg === 'foliage') {
      ctx.fillStyle = '#1A3323';
    } else if (bg === 'desert') {
      ctx.fillStyle = '#3D3020';
    } else if (bg === 'urban') {
      ctx.fillStyle = '#222831';
    } else {
      ctx.fillStyle = '#0E1318';
    }
    ctx.fillRect(0, 0, width, height);

    const zoom = editorState.zoom;
    const centerX = width / 2 + editorState.panX;
    const centerY = height / 2 + editorState.panY;

    // Center alignment offsets:
    // Even resolution: center aligns to pixel border (0.0 game px = border)
    // Odd resolution: center aligns to pixel middle (0.0 game px = center of pixel)
    const isEvenW = editorState.activeProfile.width % 2 === 0;
    const isEvenH = editorState.activeProfile.height % 2 === 0;

    // 2. Pixel Grid
    if (editorState.showGrid && zoom >= 4) {
      ctx.lineWidth = 1;
      const startX = centerX % zoom;
      const startY = centerY % zoom;

      // 1px subtle lines
      ctx.strokeStyle = '#1B232C';
      ctx.beginPath();
      for (let x = startX; x < width; x += zoom) {
        ctx.moveTo(Math.floor(x) + 0.5, 0);
        ctx.lineTo(Math.floor(x) + 0.5, height);
      }
      for (let y = startY; y < height; y += zoom) {
        ctx.moveTo(0, Math.floor(y) + 0.5);
        ctx.lineTo(width, Math.floor(y) + 0.5);
      }
      ctx.stroke();

      // 5px / 10px major lines
      ctx.strokeStyle = '#263342';
      ctx.beginPath();
      const majorStep = zoom * 5;
      const majorStartX = centerX % majorStep;
      const majorStartY = centerY % majorStep;
      for (let x = majorStartX; x < width; x += majorStep) {
        ctx.moveTo(Math.floor(x) + 0.5, 0);
        ctx.lineTo(Math.floor(x) + 0.5, height);
      }
      for (let y = majorStartY; y < height; y += majorStep) {
        ctx.moveTo(0, Math.floor(y) + 0.5);
        ctx.lineTo(width, Math.floor(y) + 0.5);
      }
      ctx.stroke();
    }

    // 3. Concentric Distance Rings (5 / 10 / 20 px)
    if (editorState.showRings) {
      ctx.strokeStyle = 'rgba(54, 242, 203, 0.15)';
      ctx.lineWidth = 1;
      ctx.setLineDash([2, 4]);
      for (const r of [5, 10, 20]) {
        ctx.beginPath();
        ctx.arc(centerX, centerY, r * zoom, 0, Math.PI * 2);
        ctx.stroke();
      }
      ctx.setLineDash([]);
    }

    // 4. Render Crosshair Layers
    const ch = editorState.crosshair;
    for (const layer of ch.layers) {
      if (!layer.visible) continue;
      ctx.save();
      // Apply layer global offset + crosshair offset
      const lx = centerX + (ch.offsetX + layer.offsetX) * zoom;
      const ly = centerY + (ch.offsetY + layer.offsetY) * zoom;
      ctx.translate(lx, ly);
      if (layer.rotation !== 0) {
        ctx.rotate((layer.rotation * Math.PI) / 180);
      }
      ctx.globalAlpha = (ch.opacity || 1.0) * (layer.opacity || 1.0);

      // Glow and Shadow visual effects (§4.6)
      if (layer.glowRadius && layer.glowColor) {
        ctx.shadowColor = layer.glowColor;
        ctx.shadowBlur = layer.glowRadius * zoom * 0.5;
        ctx.shadowOffsetX = 0;
        ctx.shadowOffsetY = 0;
      } else if (layer.shadowColor && (layer.shadowBlur || layer.shadowOffsetX || layer.shadowOffsetY)) {
        ctx.shadowColor = layer.shadowColor;
        ctx.shadowBlur = (layer.shadowBlur || 0) * zoom * 0.5;
        ctx.shadowOffsetX = (layer.shadowOffsetX || 0) * zoom;
        ctx.shadowOffsetY = (layer.shadowOffsetY || 0) * zoom;
      } else {
        ctx.shadowColor = 'transparent';
        ctx.shadowBlur = 0;
        ctx.shadowOffsetX = 0;
        ctx.shadowOffsetY = 0;
      }

      // Softening blur filter (§4.6)
      if (layer.blurRadius && layer.blurRadius > 0) {
        ctx.filter = `blur(${layer.blurRadius * (zoom / 8)}px)`;
      } else {
        ctx.filter = 'none';
      }

      ctx.fillStyle = layer.color;

      if (layer.shape.type === 'LineArms') {
        const len = (layer.shape.length || 6) * zoom;
        const thick = (layer.shape.thickness || 2) * zoom;
        // Apply dynamic spread modulation if bound (§12.5)
        const spreadDelta = Math.max(0, (editorState.simulatedSpread - 1.2) * 0.8);
        const effectiveGap = Math.max(0, (layer.shape.gap || 4) + spreadDelta);
        const gap = effectiveGap * zoom;
        const arms = layer.shape.armCount || 4;

        for (let a = 0; a < arms; a++) {
          const angle = (a * 2 * Math.PI) / arms - Math.PI / 2;
          ctx.save();
          // Apply atomic 2D arm offset
          const armOff = layer.shape.armOffsets?.[a];
          if (armOff) {
            ctx.translate(armOff.x * zoom, armOff.y * zoom);
          }
          ctx.rotate(angle + Math.PI / 2);
          // Draw arm going upward from gap
          ctx.fillRect(-thick / 2, -gap - len, thick, len);

          // Highlight selected arm if target is arm
          if (
            layer.id === editorState.selectedLayerId &&
            editorState.nudgeTarget === 'arm' &&
            a === editorState.selectedArmIndex
          ) {
            ctx.strokeStyle = '#FF4FD8';
            ctx.lineWidth = 1.5;
            ctx.strokeRect(-thick / 2 - 1.5, -gap - len - 1.5, thick + 3, len + 3);
          }
          ctx.restore();
        }
      } else if (layer.shape.type === 'TShape') {
        const len = (layer.shape.length || 6) * zoom;
        const thick = (layer.shape.thickness || 2) * zoom;
        const spreadDelta = Math.max(0, (editorState.simulatedSpread - 1.2) * 0.8);
        const effectiveGap = Math.max(0, (layer.shape.gap || 3) + spreadDelta);
        const gap = effectiveGap * zoom;
        // 3 arms: Right, Bottom, Left
        const tAngles = [Math.PI / 2, Math.PI, (3 * Math.PI) / 2];
        for (let a = 0; a < 3; a++) {
          ctx.save();
          const armOff = layer.shape.armOffsets?.[a];
          if (armOff) {
            ctx.translate(armOff.x * zoom, armOff.y * zoom);
          }
          ctx.rotate(tAngles[a]);
          ctx.fillRect(-thick / 2, -gap - len, thick, len);
          if (
            layer.id === editorState.selectedLayerId &&
            editorState.nudgeTarget === 'arm' &&
            a === editorState.selectedArmIndex
          ) {
            ctx.strokeStyle = '#FF4FD8';
            ctx.lineWidth = 1.5;
            ctx.strokeRect(-thick / 2 - 1.5, -gap - len - 1.5, thick + 3, len + 3);
          }
          ctx.restore();
        }
      } else if (layer.shape.type === 'Dot') {
        const diam = (layer.shape.diameter || 2) * zoom;
        if (layer.shape.isSquare) {
          ctx.fillRect(-diam / 2, -diam / 2, diam, diam);
        } else {
          ctx.beginPath();
          ctx.arc(0, 0, diam / 2, 0, Math.PI * 2);
          ctx.fill();
        }
      } else if (layer.shape.type === 'Ring' || layer.shape.type === 'Circle') {
        const rad = (layer.shape.radius || 6) * zoom;
        const thick = (layer.shape.thickness || 2) * zoom;
        ctx.lineWidth = thick;
        ctx.strokeStyle = layer.color;
        ctx.beginPath();
        ctx.arc(0, 0, rad, 0, Math.PI * 2);
        ctx.stroke();
      } else if (layer.shape.type === 'Chevron') {
        const len = (layer.shape.length || 7) * zoom;
        const thick = (layer.shape.thickness || 2) * zoom;
        const angleDeg = layer.shape.angle || 90;
        const halfAngle = (angleDeg * Math.PI) / 360;
        const dx = Math.sin(halfAngle) * len;
        const dy = Math.cos(halfAngle) * len;
        ctx.lineWidth = thick;
        ctx.strokeStyle = layer.color;
        ctx.lineCap = 'round';
        ctx.lineJoin = 'miter';
        ctx.beginPath();
        ctx.moveTo(-dx, dy);
        ctx.lineTo(0, 0);
        ctx.lineTo(dx, dy);
        ctx.stroke();
      } else if (layer.shape.type === 'Polygon') {
        const rad = (layer.shape.radius || 8) * zoom;
        const thick = (layer.shape.thickness || 2) * zoom;
        const points = Math.max(3, layer.shape.points || 6);
        ctx.lineWidth = thick;
        ctx.strokeStyle = layer.color;
        ctx.beginPath();
        for (let i = 0; i < points; i++) {
          const theta = (i * 2 * Math.PI) / points - Math.PI / 2;
          const px = Math.cos(theta) * rad;
          const py = Math.sin(theta) * rad;
          if (i === 0) ctx.moveTo(px, py);
          else ctx.lineTo(px, py);
        }
        ctx.closePath();
        ctx.stroke();
      } else if (layer.shape.type === 'Star') {
        const rOuter = (layer.shape.radius || 9) * zoom;
        const rInner = (layer.shape.innerRadius || 4) * zoom;
        const thick = (layer.shape.thickness || 2) * zoom;
        const points = Math.max(3, layer.shape.points || 4);
        ctx.lineWidth = thick;
        ctx.strokeStyle = layer.color;
        ctx.beginPath();
        for (let i = 0; i < points * 2; i++) {
          const r = i % 2 === 0 ? rOuter : rInner;
          const theta = (i * Math.PI) / points - Math.PI / 2;
          const px = Math.cos(theta) * r;
          const py = Math.sin(theta) * r;
          if (i === 0) ctx.moveTo(px, py);
          else ctx.lineTo(px, py);
        }
        ctx.closePath();
        ctx.stroke();
      } else if (layer.shape.type === 'SvgPath') {
        const pathData = layer.shape.pathData || 'M 0,-8 L 6,6 L 0,3 L -6,6 Z';
        try {
          const path2d = new Path2D(pathData);
          ctx.save();
          ctx.scale(zoom, zoom);
          ctx.lineWidth = (layer.shape.thickness || 2) / zoom;
          ctx.strokeStyle = layer.color;
          ctx.fillStyle = layer.color;
          ctx.stroke(path2d);
          ctx.restore();
        } catch {
          ctx.strokeStyle = '#FF3864';
          ctx.lineWidth = 2;
          ctx.beginPath();
          ctx.moveTo(-6, -6);
          ctx.lineTo(6, 6);
          ctx.moveTo(6, -6);
          ctx.lineTo(-6, 6);
          ctx.stroke();
        }
      } else if (layer.shape.type === 'Rect') {
        const rw = (layer.shape.width || 12) * zoom;
        const rh = (layer.shape.height || 8) * zoom;
        const cr = Math.min((layer.shape.cornerRadius || 0) * zoom, Math.min(rw, rh) / 2);
        if (cr > 0 && typeof ctx.roundRect === 'function') {
          ctx.beginPath();
          ctx.roundRect(-rw / 2, -rh / 2, rw, rh, cr);
          ctx.fill();
        } else {
          ctx.fillRect(-rw / 2, -rh / 2, rw, rh);
        }
      } else if (layer.shape.type === 'Arc') {
        const rad = (layer.shape.radius || 10) * zoom;
        const thick = (layer.shape.thickness || 2) * zoom;
        const startRad = ((layer.shape.startAngle || 0) * Math.PI) / 180;
        const endRad = ((layer.shape.endAngle || 180) * Math.PI) / 180;
        ctx.lineWidth = thick;
        ctx.strokeStyle = layer.color;
        ctx.beginPath();
        ctx.arc(0, 0, rad, startRad, endRad);
        ctx.stroke();
      } else if (layer.shape.type === 'Triangle') {
        const base = (layer.shape.base || 10) * zoom;
        const h = (layer.shape.height || 12) * zoom;
        ctx.beginPath();
        ctx.moveTo(0, -h / 2);
        ctx.lineTo(base / 2, h / 2);
        ctx.lineTo(-base / 2, h / 2);
        ctx.closePath();
        ctx.fill();
      } else if (layer.shape.type === 'Plus') {
        const len = (layer.shape.length || 6) * zoom;
        const thick = (layer.shape.thickness || 2) * zoom;
        const gap = (layer.shape.gap || 2) * zoom;
        const hole = (layer.shape.centerHole || 0) * zoom;
        const startOffset = Math.max(gap, hole / 2);
        ctx.fillRect(-thick / 2, -startOffset - len, thick, len); // Top
        ctx.fillRect(startOffset, -thick / 2, len, thick);         // Right
        ctx.fillRect(-thick / 2, startOffset, thick, len);        // Bottom
        ctx.fillRect(-startOffset - len, -thick / 2, len, thick); // Left
      }

      ctx.restore();
    }

    // 4b. Blank-Start Empty State (§2, §12)
    if (ch.layers.length === 0) {
      ctx.save();
      ctx.fillStyle = '#8E98A8';
      ctx.font = '600 14px "Satoshi", sans-serif';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText('Blank Canvas — 0 Layers (§2)', centerX, centerY - 36);
      ctx.font = '12px "Satoshi", sans-serif';
      ctx.fillStyle = '#545E6E';
      ctx.fillText('Add a shape from the toolbar. All parts place at (0, 0) center marker.', centerX, centerY - 16);
      ctx.restore();
    }

    // 4c. Multi-Selection Bounding Box & Outlines (§3, §12)
    if (editorState.selectedLayers.length > 0) {
      ctx.save();
      let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
      for (const selLayer of editorState.selectedLayers) {
        const lx = centerX + (ch.offsetX + selLayer.offsetX) * zoom;
        const ly = centerY + (ch.offsetY + selLayer.offsetY) * zoom;
        const boundRadius = 14 * zoom;
        minX = Math.min(minX, lx - boundRadius);
        maxX = Math.max(maxX, lx + boundRadius);
        minY = Math.min(minY, ly - boundRadius);
        maxY = Math.max(maxY, ly + boundRadius);

        // Individual selection outline
        ctx.strokeStyle = '#38BDF8';
        ctx.lineWidth = 1.5;
        ctx.setLineDash([3, 3]);
        ctx.strokeRect(lx - 12, ly - 12, 24, 24);
      }

      if (editorState.selectedLayers.length > 1 && isFinite(minX)) {
        // Multi-select bounding box (§3, §12)
        ctx.strokeStyle = '#38BDF8';
        ctx.lineWidth = 1.5;
        ctx.setLineDash([4, 4]);
        ctx.fillStyle = 'rgba(56, 189, 248, 0.05)';
        ctx.fillRect(minX, minY, maxX - minX, maxY - minY);
        ctx.strokeRect(minX, minY, maxX - minX, maxY - minY);

        // Bounding box center marker
        const bbCenterX = (minX + maxX) / 2;
        const bbCenterY = (minY + maxY) / 2;
        ctx.setLineDash([]);
        ctx.fillStyle = '#38BDF8';
        ctx.beginPath();
        ctx.arc(bbCenterX, bbCenterY, 3, 0, Math.PI * 2);
        ctx.fill();
      }
      ctx.restore();
    }

    // 4d. Marquee Selection Box (§3, §12)
    if (isMarquee) {
      ctx.save();
      const mx = Math.min(marqueeStartX, marqueeEndX);
      const my = Math.min(marqueeStartY, marqueeEndY);
      const mw = Math.abs(marqueeEndX - marqueeStartX);
      const mh = Math.abs(marqueeEndY - marqueeStartY);
      ctx.fillStyle = 'rgba(56, 189, 248, 0.12)';
      ctx.strokeStyle = '#38BDF8';
      ctx.lineWidth = 1.5;
      ctx.setLineDash([4, 4]);
      ctx.fillRect(mx, my, mw, mh);
      ctx.strokeRect(mx, my, mw, mh);
      ctx.restore();
    }

    // 5. Center Guides & Alignment Indicator (§6.8)
    if (editorState.showGuides) {
      // Magenta #FF4FD8 center dashed lines
      ctx.strokeStyle = '#FF4FD8';
      ctx.lineWidth = 1;
      ctx.setLineDash([4, 4]);

      ctx.beginPath();
      ctx.moveTo(0, Math.floor(centerY) + 0.5);
      ctx.lineTo(width, Math.floor(centerY) + 0.5);
      ctx.moveTo(Math.floor(centerX) + 0.5, 0);
      ctx.lineTo(Math.floor(centerX) + 0.5, height);
      ctx.stroke();
      ctx.setLineDash([]);

      // Center marker visibly indicating edge vs pixel-middle
      ctx.strokeStyle = '#FF4FD8';
      ctx.lineWidth = 2;
      ctx.beginPath();
      if (isEvenW && isEvenH) {
        // Edge alignment indicator (diamond / square over grid intersection)
        ctx.strokeRect(centerX - 4, centerY - 4, 8, 8);
      } else {
        // Middle alignment indicator (concentric circle)
        ctx.arc(centerX, centerY, 5, 0, Math.PI * 2);
        ctx.stroke();
      }
    }

    // 6. Rulers (Top & Left)
    if (editorState.showRulers) {
      ctx.fillStyle = '#12161B';
      ctx.fillRect(0, 0, width, 18);
      ctx.fillRect(0, 0, 18, height);

      ctx.fillStyle = '#262E38';
      ctx.fillRect(0, 17, width, 1);
      ctx.fillRect(17, 0, 1, height);

      ctx.fillStyle = '#93A0AF';
      ctx.font = '9px "JetBrains Mono", monospace';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'top';

      // Horizontal ruler ticks
      const rulerStep = zoom >= 16 ? 5 : 10;
      for (let px = -50; px <= 50; px += rulerStep) {
        const screenX = centerX + px * zoom;
        if (screenX >= 20 && screenX < width) {
          ctx.fillRect(screenX, 10, 1, 7);
          ctx.fillText(`${px}`, screenX, 1);
        }
      }

      // Vertical ruler ticks
      ctx.textAlign = 'left';
      ctx.textBaseline = 'middle';
      for (let py = -50; py <= 50; py += rulerStep) {
        const screenY = centerY + py * zoom;
        if (screenY >= 20 && screenY < height) {
          ctx.fillRect(10, screenY, 7, 1);
          ctx.fillText(`${py}`, 1, screenY);
        }
      }
    }

    ctx.restore();
  }

  let isDraggingTarget = $state(false);
  let isMarquee = $state(false);
  let marqueeStartX = $state(0);
  let marqueeStartY = $state(0);
  let marqueeEndX = $state(0);
  let marqueeEndY = $state(0);
  let dragStartClientX = $state(0);
  let dragStartClientY = $state(0);
  let initialTargetX = $state(0);
  let initialTargetY = $state(0);

  function handleMouseDown(e: MouseEvent) {
    if (e.button === 1 || e.buttons === 4 || e.altKey || (editorState.activeTool === 'select' && e.shiftKey)) {
      // Middle click or Alt+drag to pan
      isPanning = true;
      startPanX = e.clientX;
      startPanY = e.clientY;
      return;
    }

    if (e.button === 0) {
      if (!canvasEl) return;
      const rect = canvasEl.getBoundingClientRect();
      const mouseX = e.clientX - rect.left;
      const mouseY = e.clientY - rect.top;

      // Start marquee selection drag
      isMarquee = true;
      marqueeStartX = mouseX;
      marqueeStartY = mouseY;
      marqueeEndX = mouseX;
      marqueeEndY = mouseY;

      isDraggingTarget = true;
      dragStartClientX = e.clientX;
      dragStartClientY = e.clientY;

      if (editorState.nudgeTarget === 'all') {
        initialTargetX = editorState.crosshair.offsetX;
        initialTargetY = editorState.crosshair.offsetY;
      } else if (editorState.nudgeTarget === 'layer' && editorState.selectedLayer) {
        initialTargetX = editorState.selectedLayer.offsetX;
        initialTargetY = editorState.selectedLayer.offsetY;
      } else if (editorState.nudgeTarget === 'arm' && editorState.selectedLayer) {
        const armCount = editorState.selectedLayer.shape.armCount || 4;
        if (!editorState.selectedLayer.shape.armOffsets || editorState.selectedLayer.shape.armOffsets.length < armCount) {
          editorState.selectedLayer.shape.armOffsets = Array.from({ length: armCount }, () => ({ x: 0, y: 0 }));
        }
        const arm = editorState.selectedLayer.shape.armOffsets[editorState.selectedArmIndex];
        initialTargetX = arm.x;
        initialTargetY = arm.y;
      }
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (!canvasEl || !containerEl) return;
    const rect = canvasEl.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    if (isPanning) {
      editorState.panX += mouseX - startPanX;
      editorState.panY += mouseY - startPanY;
      startPanX = mouseX;
      startPanY = mouseY;
      renderCanvas();
      return;
    }

    if (isMarquee) {
      marqueeEndX = mouseX;
      marqueeEndY = mouseY;
      renderCanvas();
    }

    if (isDraggingTarget && !isMarquee) {
      const zoom = editorState.zoom;
      const deltaX = (e.clientX - dragStartClientX) / zoom;
      const deltaY = (e.clientY - dragStartClientY) / zoom;

      if (editorState.nudgeTarget === 'all') {
        editorState.crosshair.offsetX = Math.round((initialTargetX + deltaX) * 10) / 10;
        editorState.crosshair.offsetY = Math.round((initialTargetY + deltaY) * 10) / 10;
      } else if (editorState.nudgeTarget === 'layer' && editorState.selectedLayer) {
        editorState.selectedLayer.offsetX = Math.round((initialTargetX + deltaX) * 10) / 10;
        editorState.selectedLayer.offsetY = Math.round((initialTargetY + deltaY) * 10) / 10;
      } else if (editorState.nudgeTarget === 'arm' && editorState.selectedLayer) {
        const arm = editorState.selectedLayer.shape.armOffsets?.[editorState.selectedArmIndex];
        if (arm) {
          arm.x = Math.round((initialTargetX + deltaX) * 10) / 10;
          arm.y = Math.round((initialTargetY + deltaY) * 10) / 10;
        }
      }
      renderCanvas();
      return;
    }

    const centerX = rect.width / 2 + editorState.panX;
    const centerY = rect.height / 2 + editorState.panY;
    const zoom = editorState.zoom;

    hoverGameX = Math.round(((mouseX - centerX) / zoom) * 10) / 10;
    hoverGameY = Math.round(((mouseY - centerY) / zoom) * 10) / 10;

    const profileW = editorState.activeProfile.width;
    const profileH = editorState.activeProfile.height;
    hoverPixelI = Math.floor(profileW / 2 + hoverGameX);
    hoverPixelJ = Math.floor(profileH / 2 + hoverGameY);
  }

  function handleMouseUp(e: MouseEvent) {
    if (isMarquee) {
      const minX = Math.min(marqueeStartX, marqueeEndX);
      const maxX = Math.max(marqueeStartX, marqueeEndX);
      const minY = Math.min(marqueeStartY, marqueeEndY);
      const maxY = Math.max(marqueeStartY, marqueeEndY);

      if (Math.abs(maxX - minX) > 8 || Math.abs(maxY - minY) > 8) {
        if (canvasEl) {
          const rect = canvasEl.getBoundingClientRect();
          const centerX = rect.width / 2 + editorState.panX;
          const centerY = rect.height / 2 + editorState.panY;
          const zoom = editorState.zoom;

          const matched = editorState.crosshair.layers.filter((l) => {
            const lx = centerX + (editorState.crosshair.offsetX + l.offsetX) * zoom;
            const ly = centerY + (editorState.crosshair.offsetY + l.offsetY) * zoom;
            return lx >= minX && lx <= maxX && ly >= minY && ly <= maxY;
          });

          if (matched.length > 0) {
            editorState.selectedLayerIds = matched.map((m) => m.id);
          }
        }
      }
      isMarquee = false;
      renderCanvas();
    }

    if (isDraggingTarget) {
      editorState.pushUndo();
      isDraggingTarget = false;
    }
    isPanning = false;
  }

  function handleToolSelect(toolId: ToolType) {
    editorState.activeTool = toolId;
    if (toolId === 'select') {
      editorState.nudgeTarget = 'layer';
    } else if (toolId === 'direct') {
      editorState.nudgeTarget = 'arm';
    } else if (toolId === 'lines') {
      editorState.addLayer('LineArms', 'Line Arms');
    } else if (toolId === 'dot') {
      editorState.addLayer('Dot', 'Center Dot');
    } else if (toolId === 'rect') {
      editorState.addLayer('TShape', 'T-Shape');
    } else if (toolId === 'circle') {
      editorState.addLayer('Circle', 'Circle / Ring');
    } else if (toolId === 'pen') {
      editorState.addLayer('SvgPath', 'Custom Path');
    }
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    const zoomFactors = [1, 2, 4, 8, 16, 32, 64];
    const currentIdx = zoomFactors.indexOf(editorState.zoom);
    if (e.deltaY < 0) {
      // Zoom in
      if (currentIdx < zoomFactors.length - 1) {
        editorState.zoom = zoomFactors[currentIdx + 1];
      }
    } else {
      // Zoom out
      if (currentIdx > 0) {
        editorState.zoom = zoomFactors[currentIdx - 1];
      }
    }
    renderCanvas();
  }

  $effect(() => {
    // Re-render when any relevant state changes
    const _ = [
      editorState.crosshair,
      editorState.zoom,
      editorState.panX,
      editorState.panY,
      editorState.showGrid,
      editorState.showRulers,
      editorState.showGuides,
      editorState.showRings,
      editorState.activeProfile,
      editorState.simulatedSpread,
    ];
    renderCanvas();
  });

  onMount(() => {
    renderCanvas();
    const resizeObserver = new ResizeObserver(() => {
      renderCanvas();
    });
    if (containerEl) {
      resizeObserver.observe(containerEl);
    }
    return () => resizeObserver.disconnect();
  });
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions a11y_no_noninteractive_tabindex -->
<div
  class="canvas-container"
  bind:this={containerEl}
  onmousemove={handleMouseMove}
  onmousedown={handleMouseDown}
  onmouseup={handleMouseUp}
  onwheel={handleWheel}
  role="region"
  tabindex="-1"
  aria-label="Interactive Crosshair Canvas"
>
  <canvas bind:this={canvasEl} class="pixel-canvas"></canvas>

  <!-- Floating Tool Palette (Left) -->
  <div class="tool-palette">
    {#each tools as tool}
      <Tooltip title={tool.tip} shortcut={tool.shortcut}>
        <button
          class="btn btn-icon mono tool-btn {editorState.activeTool === tool.id ? 'btn-selected' : ''}"
          onclick={() => handleToolSelect(tool.id)}
          aria-label="Tool {tool.label}"
        >
          {tool.label}
        </button>
      </Tooltip>
    {/each}
  </div>

  <!-- Floating View Controls Bar (Bottom) -->
  <div class="view-controls mono">
    <!-- Zoom Selector -->
    <div class="chips-group">
      {#each [1, 2, 4, 8, 16, 32] as z}
        <button
          class="pill-chip {editorState.zoom === z ? 'active' : ''}"
          onclick={() => {
            editorState.zoom = z;
            renderCanvas();
          }}
        >
          {z}×
        </button>
      {/each}
      <button
        class="pill-chip"
        onclick={() => {
          editorState.zoom = 1;
          editorState.panX = 0;
          editorState.panY = 0;
          renderCanvas();
        }}
      >
        1:1
      </button>
    </div>

    <!-- Feature Toggles -->
    <div class="chips-group">
      <button
        class="pill-chip {editorState.showGrid ? 'active' : ''}"
        onclick={() => (editorState.showGrid = !editorState.showGrid)}
      >
        Grid
      </button>
      <button
        class="pill-chip {editorState.showRulers ? 'active' : ''}"
        onclick={() => (editorState.showRulers = !editorState.showRulers)}
      >
        Rulers
      </button>
      <button
        class="pill-chip {editorState.showGuides ? 'active' : ''}"
        onclick={() => (editorState.showGuides = !editorState.showGuides)}
      >
        Guides
      </button>
    </div>

    <!-- Tactical Preview Background (§6.8) -->
    <div class="chips-group">
      <span class="mono" style="font-size: 10px; color: var(--text-muted); align-self: center; margin-right: 2px;">BG:</span>
      {#each (['dark', 'light', 'foliage', 'desert', 'urban'] as const) as b}
        <button
          class="pill-chip {editorState.previewBackground === b ? 'active' : ''}"
          onclick={() => {
            editorState.setPreviewBackground(b);
            renderCanvas();
          }}
        >
          {b.toUpperCase()}
        </button>
      {/each}
    </div>

    <!-- Coordinate Readout -->
    <div class="pill-readout">
      x {hoverGameX >= 0 ? `+${hoverGameX.toFixed(1)}` : hoverGameX.toFixed(1)} y {hoverGameY >= 0 ? `+${hoverGameY.toFixed(1)}` : hoverGameY.toFixed(1)}
      <span style="color: var(--text-muted); margin-left: 4px;">({hoverPixelI}, {hoverPixelJ})</span>
    </div>

    <!-- Center Alignment Diagnostic Status -->
    <div
      class="pill-readout {editorState.alignmentDiagnostic.isEvenWidth ? 'badge-warning' : 'badge'}"
    >
      {editorState.alignmentDiagnostic.message}
    </div>
  </div>
</div>

<style>
  .canvas-container {
    flex: 1;
    position: relative;
    min-width: 0;
    overflow: hidden;
    background: #0E1318;
  }

  .pixel-canvas {
    display: block;
    width: 100%;
    height: 100%;
    cursor: crosshair;
  }

  .tool-palette {
    position: absolute;
    left: 12px;
    top: 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    background: rgba(18, 22, 27, 0.85);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 6px;
    backdrop-filter: blur(8px);
    z-index: 10;
  }

  .tool-btn {
    width: 32px;
    height: 32px;
  }

  .view-controls {
    position: absolute;
    left: 64px;
    bottom: 12px;
    right: 12px;
    display: flex;
    align-items: center;
    gap: 12px;
    z-index: 10;
    pointer-events: none;
  }

  .chips-group {
    display: flex;
    gap: 4px;
    background: rgba(18, 22, 27, 0.85);
    border: 1px solid var(--border);
    border-radius: var(--radius-pill);
    padding: 3px 6px;
    pointer-events: auto;
    backdrop-filter: blur(8px);
  }

  .pill-chip {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 11px;
    padding: 2px 8px;
    border-radius: var(--radius-pill);
    cursor: pointer;
    font-family: var(--font-mono);
  }

  .pill-chip:hover {
    color: var(--text);
  }

  .pill-chip.active {
    background: var(--accent-dim);
    color: var(--accent);
    font-weight: 600;
  }

  .pill-readout {
    background: rgba(18, 22, 27, 0.85);
    border: 1px solid var(--border);
    border-radius: var(--radius-pill);
    padding: 4px 12px;
    font-size: 11px;
    color: var(--text);
    pointer-events: auto;
    backdrop-filter: blur(8px);
    white-space: nowrap;
  }
</style>
