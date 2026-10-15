import type {
  Crosshair,
  Layer,
  ShapeType,
  ShapeData,
  PropertyDescriptor,
  ResolutionProfile,
  PreviewBackground,
  ToolType,
  NudgeTarget,
  NudgeFrame,
  AlignmentDiagnostic,
  AlignmentIssue,
  AnimTrack,
  GunProfile,
  DynamicBinding,
} from './types';
import propertiesData from '../generated/properties.json';

export const DESCRIPTORS: PropertyDescriptor[] = propertiesData.descriptors as PropertyDescriptor[];

export const DEFAULT_GUN_PROFILES: GunProfile[] = [
  {
    id: 'gun-rifle-01',
    name: 'Rifle (Template)',
    notes: 'Example placeholder curve - not real game data (§12.4)',
    fire_rate_hz: 11.0,
    magazine: 30,
    spread: {
      base: 1.2,
      per_shot: 0.9,
      max: 14.0,
      recovery_per_sec: 18.0,
      move_penalty: 5.0,
      shift_move_penalty: 2.0,
      crouch_bonus: 0.5,
      air_penalty: 9.0,
    },
    smoothing: {
      attack_ms: 20.0,
      release_ms: 120.0,
    },
    pattern: Array.from({ length: 30 }, (_, i) => {
      const kick_y = Math.min(i * 2.5, 28.0);
      const kick_x = i < 8 ? 0.0 : i < 18 ? (i - 8) * 1.4 : 14.0 - (i - 18) * 2.2;
      return { x: kick_x, y: kick_y };
    }),
  },
  {
    id: 'gun-smg-02',
    name: 'SMG (Template)',
    notes: 'Example placeholder curve - not real game data (§12.4)',
    fire_rate_hz: 15.0,
    magazine: 30,
    spread: {
      base: 1.8,
      per_shot: 0.6,
      max: 18.0,
      recovery_per_sec: 22.0,
      move_penalty: 3.5,
      shift_move_penalty: 1.2,
      crouch_bonus: 0.3,
      air_penalty: 7.0,
    },
    smoothing: {
      attack_ms: 10.0,
      release_ms: 100.0,
    },
    pattern: Array.from({ length: 30 }, (_, i) => {
      const kick_y = Math.min(i * 1.8, 20.0);
      const kick_x = Math.sin(i * 1.2) * 8.0;
      return { x: kick_x, y: kick_y };
    }),
  },
  {
    id: 'gun-pistol-03',
    name: 'Pistol (Template)',
    notes: 'Example placeholder curve - not real game data (§12.4)',
    fire_rate_hz: 6.0,
    magazine: 12,
    spread: {
      base: 0.8,
      per_shot: 1.4,
      max: 10.0,
      recovery_per_sec: 25.0,
      move_penalty: 2.0,
      shift_move_penalty: 0.8,
      crouch_bonus: 0.2,
      air_penalty: 6.0,
    },
    smoothing: {
      attack_ms: 15.0,
      release_ms: 80.0,
    },
    pattern: Array.from({ length: 12 }, (_, i) => ({
      x: 0,
      y: Math.min(i * 3.0, 15.0),
    })),
  },
];

export const RESOLUTION_PROFILES: ResolutionProfile[] = [
  {
    id: '1080p-native',
    name: '1920×1080 · Native',
    width: 1920,
    height: 1080,
    isStretched: false,
    isNative: true,
    dpiScale: 1.0,
  },
  {
    id: '1440p-native',
    name: '2560×1440 · Native',
    width: 2560,
    height: 1440,
    isStretched: false,
    isNative: true,
    dpiScale: 1.0,
  },
  {
    id: '4-3-stretched',
    name: '1280×960 · 4:3 Stretched',
    width: 1280,
    height: 960,
    isStretched: true,
    isNative: false,
    dpiScale: 1.0,
  },
];

export const INITIAL_CROSSHAIR: Crosshair = {
  id: 'ch-cyan-gap-001',
  name: 'Cyan Gap',
  description: 'Clean competitive 4-arm rifle crosshair with 4px gap and 2px thickness',
  author: 'ProPlayer',
  tags: ['Rifle', 'Competitive', 'Cyan'],
  favorite: true,
  formatVersion: 1,
  offsetX: 0,
  offsetY: 0,
  scale: 1.0,
  opacity: 1.0,
  layers: [
    {
      id: 'layer-arms-1',
      name: 'Arms ×4',
      visible: true,
      locked: false,
      shape: {
        type: 'LineArms',
        length: 6,
        thickness: 2,
        gap: 4,
        armCount: 4,
      },
      color: '#36F2CB',
      opacity: 1.0,
      offsetX: 0,
      offsetY: 0,
      rotation: 0,
    },
    {
      id: 'layer-dot-2',
      name: 'Center dot',
      visible: true,
      locked: false,
      shape: {
        type: 'Dot',
        diameter: 2,
        isSquare: true,
      },
      color: '#36F2CB',
      opacity: 1.0,
      offsetX: 0,
      offsetY: 0,
      rotation: 0,
    },
    {
      id: 'layer-glow-3',
      name: 'Glow',
      visible: false,
      locked: false,
      shape: {
        type: 'LineArms',
        length: 7,
        thickness: 3,
        gap: 3.5,
        armCount: 4,
      },
      color: '#0F2A26',
      opacity: 0.6,
      offsetX: 0,
      offsetY: 0,
      rotation: 0,
      glowRadius: 4,
      glowColor: '#36F2CB',
    },
  ],
};

export const PRESET_LIBRARY: Crosshair[] = [
  INITIAL_CROSSHAIR,
  {
    id: 'ch-tight-plus-002',
    name: 'Tight Plus',
    description: 'Compact 4px arms with zero gap for precision one-taps',
    author: 'TenZ',
    tags: ['Pistol', 'Tight'],
    favorite: false,
    formatVersion: 1,
    offsetX: 0,
    offsetY: 0,
    scale: 1.0,
    opacity: 1.0,
    layers: [
      {
        id: 'tp-arms',
        name: 'Inner Arms',
        visible: true,
        locked: false,
        shape: { type: 'LineArms', length: 4, thickness: 2, gap: 1, armCount: 4 },
        color: '#36F2CB',
        opacity: 1.0,
        offsetX: 0,
        offsetY: 0,
        rotation: 0,
      },
      {
        id: 'tp-dot',
        name: 'Center Dot',
        visible: true,
        locked: false,
        shape: { type: 'Dot', diameter: 2, isSquare: true },
        color: '#36F2CB',
        opacity: 1.0,
        offsetX: 0,
        offsetY: 0,
        rotation: 0,
      },
    ],
  },
  {
    id: 'ch-open-dot-003',
    name: 'Open Dot',
    description: 'Sub-pixel 3px round dot with wide 8px peripheral brackets',
    author: 'Yay',
    tags: ['Operator', 'Dot'],
    favorite: false,
    formatVersion: 1,
    offsetX: 0,
    offsetY: 0,
    scale: 1.0,
    opacity: 1.0,
    layers: [
      {
        id: 'od-dot',
        name: 'Center Dot',
        visible: true,
        locked: false,
        shape: { type: 'Dot', diameter: 3, isSquare: false },
        color: '#36F2CB',
        opacity: 1.0,
        offsetX: 0,
        offsetY: 0,
        rotation: 0,
      },
      {
        id: 'od-arms',
        name: 'Outer Brackets',
        visible: true,
        locked: false,
        shape: { type: 'LineArms', length: 3, thickness: 1, gap: 8, armCount: 4 },
        color: '#36F2CB',
        opacity: 0.8,
        offsetX: 0,
        offsetY: 0,
        rotation: 0,
      },
    ],
  },
  {
    id: 'ch-long-arms-004',
    name: 'Long Arms',
    description: 'Extended 9px arms with 4px gap for sniper lining',
    author: 'Derke',
    tags: ['Sniper', 'Long'],
    favorite: false,
    formatVersion: 1,
    offsetX: 0,
    offsetY: 0,
    scale: 1.0,
    opacity: 1.0,
    layers: [
      {
        id: 'la-arms',
        name: 'Cross Lines',
        visible: true,
        locked: false,
        shape: { type: 'LineArms', length: 9, thickness: 2, gap: 4, armCount: 4 },
        color: '#36F2CB',
        opacity: 1.0,
        offsetX: 0,
        offsetY: 0,
        rotation: 0,
      },
    ],
  },
];

let idCounter = 0;
export function uniqueId(prefix: string): string {
  idCounter += 1;
  return `${prefix}-${Date.now()}-${idCounter}`;
}

export class EditorState {
  crosshair = $state<Crosshair>(JSON.parse(JSON.stringify(INITIAL_CROSSHAIR)));
  library = $state<Crosshair[]>(JSON.parse(JSON.stringify(PRESET_LIBRARY)));
  selectedLayerId = $state<string>('layer-arms-1');
  selectedArmIndex = $state<number>(0);
  activeTool = $state<ToolType>('select');

  // Zoom & Viewport
  zoom = $state<number>(16);
  panX = $state<number>(0);
  panY = $state<number>(0);
  showGrid = $state<boolean>(true);
  showRulers = $state<boolean>(true);
  showGuides = $state<boolean>(true);
  showRings = $state<boolean>(true);
  showBounds = $state<boolean>(false);
  previewBackground = $state<PreviewBackground>('dark');

  // Modals & Panels
  importExportModalOpen = $state<boolean>(false);
  importExportTab = $state<'import' | 'export'>('import');
  resolutionModalOpen = $state<boolean>(false);

  // Nudge Control
  nudgeStep = $state<number>(0.5);
  nudgeTarget = $state<NudgeTarget>('layer');
  nudgeFrame = $state<NudgeFrame>('screen');

  // Active Profile & System Flags
  activeProfile = $state<ResolutionProfile>(RESOLUTION_PROFILES[0]);
  overlayEnabled = $state<boolean>(true);
  mcpEnabled = $state<boolean>(true);
  isAgentEditing = $state<boolean>(false);
  mcpPanelOpen = $state<boolean>(false);
  mcpActivityLog = $state<Array<{ id: string; timestamp: string; tool: string; summary: string; status: string }>>([
    {
      id: 'log-init',
      timestamp: new Date().toLocaleTimeString(),
      tool: 'initialize',
      summary: 'MCP server handshake connected',
      status: 'success',
    },
  ]);
  private agentEditingTimer: ReturnType<typeof setTimeout> | null = null;
  dynamicReticleEnabled = $state<boolean>(false);

  // Dynamic Reticle & Spray Lab (§12, M9)
  sprayLabOpen = $state<boolean>(false);
  dynamicOptInOpen = $state<boolean>(false);
  compliancePanelOpen = $state<boolean>(false);
  gunProfiles = $state<GunProfile[]>(JSON.parse(JSON.stringify(DEFAULT_GUN_PROFILES)));
  activeGunProfile = $state<GunProfile>(JSON.parse(JSON.stringify(DEFAULT_GUN_PROFILES[0])));
  dynamicBindings = $state<DynamicBinding[]>([
    {
      id: 'bind-gap-spread',
      property_path: 'layers[0].gap',
      driver: 'spread',
      in_range: [1.0, 10.0],
      out_range: [0.0, 6.0],
      curve: 'linear',
      mode: 'add',
    },
  ]);
  simulatedSpread = $state<number>(1.2);
  simulatedSpeed = $state<number>(0.0);

  // Command Palette & Modals
  commandPaletteOpen = $state<boolean>(false);

  // Calibration Wizard & Screenshot Align (§6.7, M8)
  calibrationOpen = $state<boolean>(false);
  calibrationStep = $state<number>(1);
  calibrationPatternActive = $state<boolean>(false);
  calibrationOffset = $state<{ x: number; y: number }>({ x: 0, y: 0 });
  screenshotAlignMode = $state<boolean>(false);
  screenshotDetectedOffset = $state<{ x: number; y: number } | null>(null);
  screenshotConfidence = $state<number>(0);

  // Animation & Timeline (§4.7)
  currentTime = $state<number>(0.0);
  isPlaying = $state<boolean>(false);
  timelineDuration = $state<number>(3.0);
  loopAnimation = $state<boolean>(true);
  staticOnlyLock = $state<boolean>(false);
  timelineOpen = $state<boolean>(false);

  // Undo / Redo history
  undoStack = $state<string[]>([]);
  redoStack = $state<string[]>([]);

  // Derived selected layer
  selectedLayer = $derived(
    this.crosshair.layers.find((l) => l.id === this.selectedLayerId) || this.crosshair.layers[0]
  );

  // Alignment Diagnostic
  alignmentDiagnostic = $derived.by<AlignmentDiagnostic>(() => {
    const isEvenW = this.activeProfile.width % 2 === 0;
    const isEvenH = this.activeProfile.height % 2 === 0;
    const centerMode = isEvenW ? 'pixel-edge' : 'pixel-middle';
    const subpixel =
      Math.abs(this.crosshair.offsetX % 1) > 0.001 || Math.abs(this.crosshair.offsetY % 1) > 0.001;

    let msg = `Center = ${centerMode === 'pixel-edge' ? 'pixel edge (even W)' : 'pixel middle (odd W)'}`;
    if (subpixel) {
      msg += ' · Subpixel offset active';
    }

    return {
      isEvenWidth: isEvenW,
      isEvenHeight: isEvenH,
      centerEdgeOrMiddle: centerMode,
      message: msg,
      hasSubpixelOffset: subpixel,
    };
  });

  // Derived alignment issues for all layers (§6.2)
  alignmentIssues = $derived.by<AlignmentIssue[]>(() => {
    const issues: AlignmentIssue[] = [];
    const w = this.activeProfile.width;
    const h = this.activeProfile.height;

    for (const layer of this.crosshair.layers) {
      if (!layer.visible) continue;
      if (layer.shape.type === 'LineArms' && layer.shape.thickness !== undefined) {
        const t = Math.round(layer.shape.thickness);
        const hAligned = (w + t) % 2 === 0;
        const vAligned = (h + t) % 2 === 0;
        if (!hAligned || !vAligned) {
          issues.push({
            layer_id: layer.id,
            layer_name: layer.name,
            axis: !hAligned && !vAligned ? 'both' : !hAligned ? 'horizontal' : 'vertical',
            edge_position: w / 2 - t / 2,
            suggested_fix: {
              type: 'adjust_thickness',
              new_thickness: Math.max(1, t + 1),
            },
          });
        }
      } else if (layer.shape.type === 'Dot' && layer.shape.diameter !== undefined) {
        const d = Math.round(layer.shape.diameter);
        const hAligned = (w + d) % 2 === 0;
        const vAligned = (h + d) % 2 === 0;
        if (!hAligned || !vAligned) {
          issues.push({
            layer_id: layer.id,
            layer_name: layer.name,
            axis: 'both',
            edge_position: w / 2 - d / 2,
            suggested_fix: {
              type: 'adjust_thickness',
              new_thickness: Math.max(1, d + 1),
            },
          });
        }
      }
    }
    return issues;
  });

  // Calculate visibility score against active tactical background (§6.8)
  visibilityScore = $derived.by<number>(() => {
    if (!this.selectedLayer) return 7.5;
    const hex = this.selectedLayer.color.replace('#', '');
    const r = parseInt(hex.substring(0, 2), 16) || 0;
    const g = parseInt(hex.substring(2, 4), 16) || 0;
    const b = parseInt(hex.substring(4, 6), 16) || 0;
    // Relative luminance calculation
    const lum = 0.2126 * (r / 255) + 0.7152 * (g / 255) + 0.0722 * (b / 255);
    let bgLum = 0.005; // 'dark' (#0E1318)
    if (this.previewBackground === 'light') {
      bgLum = 0.88; // #F1F5F9
    } else if (this.previewBackground === 'foliage') {
      bgLum = 0.04; // #1A3323
    } else if (this.previewBackground === 'desert') {
      bgLum = 0.07; // #3D3020
    } else if (this.previewBackground === 'urban') {
      bgLum = 0.03; // #222831
    }
    const maxLum = Math.max(lum, bgLum);
    const minLum = Math.min(lum, bgLum);
    const ratio = (maxLum + 0.05) / (minLum + 0.05);
    return Math.round(ratio * 10) / 10;
  });

  // Record undo state before modifying
  pushUndo() {
    this.undoStack.push(JSON.stringify(this.crosshair));
    if (this.undoStack.length > 50) {
      this.undoStack.shift();
    }
    this.redoStack = [];
  }

  undo() {
    if (this.undoStack.length === 0) return;
    const prev = this.undoStack.pop()!;
    this.redoStack.push(JSON.stringify(this.crosshair));
    this.crosshair = JSON.parse(prev);
  }

  redo() {
    if (this.redoStack.length === 0) return;
    const next = this.redoStack.pop()!;
    this.undoStack.push(JSON.stringify(this.crosshair));
    this.crosshair = JSON.parse(next);
  }

  notifyAgentActivity(tool: string, summary: string, status: string = 'success') {
    this.isAgentEditing = true;
    this.mcpActivityLog.unshift({
      id: `act-${Date.now()}-${Math.random().toString(36).substring(2, 6)}`,
      timestamp: new Date().toLocaleTimeString(),
      tool,
      summary,
      status,
    });
    if (this.mcpActivityLog.length > 100) {
      this.mcpActivityLog.pop();
    }
    if (this.agentEditingTimer) {
      clearTimeout(this.agentEditingTimer);
    }
    this.agentEditingTimer = setTimeout(() => {
      this.isAgentEditing = false;
    }, 3000);
  }

  clearMcpLog() {
    this.mcpActivityLog = [];
  }

  // Timeline and Animation methods (§4.7)
  togglePlay() {
    if (this.staticOnlyLock) {
      this.isPlaying = false;
      return;
    }
    this.isPlaying = !this.isPlaying;
    if (this.isPlaying) {
      let lastTimestamp = performance.now();
      const tick = () => {
        if (!this.isPlaying || this.staticOnlyLock) {
          this.isPlaying = false;
          return;
        }
        const now = performance.now();
        const delta = (now - lastTimestamp) / 1000;
        lastTimestamp = now;

        this.currentTime += delta;
        if (this.currentTime >= this.timelineDuration) {
          if (this.loopAnimation) {
            this.currentTime = 0;
          } else {
            this.currentTime = this.timelineDuration;
            this.isPlaying = false;
            return;
          }
        }
        requestAnimationFrame(tick);
      };
      requestAnimationFrame(tick);
    }
  }

  seek(time: number) {
    this.currentTime = Math.max(0, Math.min(time, this.timelineDuration));
  }

  toggleStaticOnlyLock() {
    this.staticOnlyLock = !this.staticOnlyLock;
    if (this.staticOnlyLock) {
      this.isPlaying = false;
      this.currentTime = 0;
    }
  }

  addTrack(track: AnimTrack) {
    if (!this.crosshair.animation) {
      this.crosshair.animation = [];
    }
    this.pushUndo();
    this.crosshair.animation.push(track);
  }

  removeTrack(id: string) {
    if (!this.crosshair.animation) return;
    this.pushUndo();
    this.crosshair.animation = this.crosshair.animation.filter((t) => t.id !== id);
  }

  // Crosshair Selection & CRUD
  selectCrosshair(id: string) {
    const item = this.library.find((c) => c.id === id);
    if (item) {
      this.pushUndo();
      this.crosshair = JSON.parse(JSON.stringify(item));
      this.selectedLayerId = this.crosshair.layers[0]?.id ?? '';
    }
  }

  newCrosshair() {
    this.pushUndo();
    const newId = uniqueId('ch');
    const created: Crosshair = {
      id: newId,
      name: `New Crosshair ${this.library.length + 1}`,
      description: 'Custom created crosshair',
      author: 'User',
      tags: ['Custom'],
      favorite: false,
      formatVersion: 1,
      offsetX: 0,
      offsetY: 0,
      scale: 1.0,
      opacity: 1.0,
      layers: [
        {
          id: uniqueId('layer'),
          name: 'Arms ×4',
          visible: true,
          locked: false,
          shape: { type: 'LineArms', length: 5, thickness: 2, gap: 3, armCount: 4 },
          color: '#36F2CB',
          opacity: 1.0,
          offsetX: 0,
          offsetY: 0,
          rotation: 0,
        },
      ],
    };
    this.library.push(created);
    this.crosshair = JSON.parse(JSON.stringify(created));
    this.selectedLayerId = created.layers[0].id;
  }

  // Nudge in 8 directions with atomic element precision (§9.4)
  nudge(dx: number, dy: number) {
    this.pushUndo();
    const stepX = dx * this.nudgeStep;
    const stepY = dy * this.nudgeStep;

    if (this.nudgeTarget === 'all') {
      this.crosshair.offsetX = Math.round((this.crosshair.offsetX + stepX) * 100) / 100;
      this.crosshair.offsetY = Math.round((this.crosshair.offsetY + stepY) * 100) / 100;
    } else if (this.nudgeTarget === 'layer' && this.selectedLayer) {
      this.selectedLayer.offsetX = Math.round((this.selectedLayer.offsetX + stepX) * 100) / 100;
      this.selectedLayer.offsetY = Math.round((this.selectedLayer.offsetY + stepY) * 100) / 100;
    } else if (this.nudgeTarget === 'arm' && this.selectedLayer) {
      const armCount = this.selectedLayer.shape.armCount || 4;
      if (!this.selectedLayer.shape.armOffsets || this.selectedLayer.shape.armOffsets.length < armCount) {
        this.selectedLayer.shape.armOffsets = Array.from({ length: armCount }, () => ({ x: 0, y: 0 }));
      }
      const armIdx = Math.max(0, Math.min(this.selectedArmIndex, armCount - 1));
      const arm = this.selectedLayer.shape.armOffsets[armIdx];
      arm.x = Math.round((arm.x + stepX) * 100) / 100;
      arm.y = Math.round((arm.y + stepY) * 100) / 100;
    }
  }

  resetNudge() {
    this.pushUndo();
    if (this.nudgeTarget === 'all') {
      this.crosshair.offsetX = 0;
      this.crosshair.offsetY = 0;
    } else if (this.nudgeTarget === 'layer' && this.selectedLayer) {
      this.selectedLayer.offsetX = 0;
      this.selectedLayer.offsetY = 0;
    } else if (this.nudgeTarget === 'arm' && this.selectedLayer && this.selectedLayer.shape.armOffsets) {
      if (this.selectedLayer.shape.armOffsets[this.selectedArmIndex]) {
        this.selectedLayer.shape.armOffsets[this.selectedArmIndex] = { x: 0, y: 0 };
      }
    }
  }

  selectArm(armIndex: number) {
    this.selectedArmIndex = armIndex;
    this.nudgeTarget = 'arm';
  }

  addLayer(type: ShapeType, name?: string) {
    this.pushUndo();
    const id = uniqueId('layer');
    const layerColor = this.crosshair.layers[0]?.color || '#00FFFF';

    let shape: ShapeData;
    let defaultName = name;

    switch (type) {
      case 'Dot':
        shape = { type: 'Dot', diameter: 2, isSquare: false };
        defaultName ||= 'Center Dot';
        break;
      case 'Circle':
        shape = { type: 'Circle', radius: 8, thickness: 2 };
        defaultName ||= 'Target Circle';
        break;
      case 'Ring':
        shape = { type: 'Ring', radius: 6, thickness: 1 };
        defaultName ||= 'Inner Ring';
        break;
      case 'TShape':
        shape = {
          type: 'TShape',
          length: 6,
          thickness: 2,
          gap: 3,
          armCount: 3,
          armOffsets: [{ x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }],
        };
        defaultName ||= 'T-Shape';
        break;
      case 'Chevron':
        shape = { type: 'Chevron', length: 7, thickness: 2, angle: 90 };
        defaultName ||= 'Chevron Arrow';
        break;
      case 'Polygon':
        shape = { type: 'Polygon', radius: 8, points: 6, thickness: 2 };
        defaultName ||= 'Hexagon Reticle';
        break;
      case 'Star':
        shape = { type: 'Star', radius: 9, innerRadius: 4, points: 4, thickness: 2 };
        defaultName ||= 'Star Reticle';
        break;
      case 'SvgPath':
        shape = {
          type: 'SvgPath',
          pathData: 'M 0,-8 L 6,6 L 0,3 L -6,6 Z',
          thickness: 2,
          innerOffset: 1.0,
        };
        defaultName ||= 'Custom Vector';
        break;
      case 'Rect':
        shape = { type: 'Rect', width: 12, height: 8, cornerRadius: 0 };
        defaultName ||= 'Rectangle / Box';
        break;
      case 'Arc':
        shape = { type: 'Arc', radius: 10, thickness: 2, startAngle: 0, endAngle: 180 };
        defaultName ||= 'Arc Segment';
        break;
      case 'Triangle':
        shape = { type: 'Triangle', base: 10, height: 12 };
        defaultName ||= 'Triangle';
        break;
      case 'Plus':
        shape = { type: 'Plus', length: 6, thickness: 2, gap: 2, centerHole: 0 };
        defaultName ||= 'Plus Cross';
        break;
      case 'Cross':
      case 'LineArms':
      default:
        shape = {
          type: 'LineArms',
          length: 6,
          thickness: 2,
          gap: 4,
          armCount: 4,
          armOffsets: [{ x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }],
        };
        defaultName ||= 'Line Arms';
        break;
    }

    const newLayer: Layer = {
      id,
      name: defaultName,
      visible: true,
      locked: false,
      shape,
      color: layerColor,
      opacity: 1.0,
      offsetX: 0,
      offsetY: 0,
      rotation: 0,
    };

    this.crosshair.layers.push(newLayer);
    this.selectedLayerId = id;
  }

  // Layer Mutations
  toggleLayerVisibility(layerId: string) {
    const layer = this.crosshair.layers.find((l) => l.id === layerId);
    if (layer) {
      this.pushUndo();
      layer.visible = !layer.visible;
    }
  }

  toggleLayerLock(layerId: string) {
    const layer = this.crosshair.layers.find((l) => l.id === layerId);
    if (layer) {
      this.pushUndo();
      layer.locked = !layer.locked;
    }
  }

  duplicateLayer(layerId: string) {
    const idx = this.crosshair.layers.findIndex((l) => l.id === layerId);
    if (idx !== -1) {
      this.pushUndo();
      const orig = this.crosshair.layers[idx];
      const cloned: Layer = JSON.parse(JSON.stringify(orig));
      cloned.id = uniqueId('layer');
      cloned.name = `${orig.name} (Copy)`;
      this.crosshair.layers.splice(idx + 1, 0, cloned);
      this.selectedLayerId = cloned.id;
    }
  }

  deleteLayer(layerId: string) {
    if (this.crosshair.layers.length <= 1) return; // Keep at least one layer
    const idx = this.crosshair.layers.findIndex((l) => l.id === layerId);
    if (idx !== -1) {
      this.pushUndo();
      this.crosshair.layers.splice(idx, 1);
      this.selectedLayerId = this.crosshair.layers[Math.max(0, idx - 1)].id;
    }
  }

  reorderLayer(fromIndex: number, toIndex: number) {
    if (
      fromIndex < 0 ||
      fromIndex >= this.crosshair.layers.length ||
      toIndex < 0 ||
      toIndex >= this.crosshair.layers.length
    )
      return;
    this.pushUndo();
    const [moved] = this.crosshair.layers.splice(fromIndex, 1);
    this.crosshair.layers.splice(toIndex, 0, moved);
  }

  updateProperty(propName: string, value: number) {
    if (!this.selectedLayer) return;
    this.pushUndo();

    if ((propName === 'line.length' || propName === 'shape.length') && this.selectedLayer.shape.length !== undefined) {
      this.selectedLayer.shape.length = value;
    } else if ((propName === 'line.thickness' || propName === 'shape.thickness') && this.selectedLayer.shape.thickness !== undefined) {
      this.selectedLayer.shape.thickness = value;
    } else if ((propName === 'line.gap' || propName === 'shape.gap') && this.selectedLayer.shape.gap !== undefined) {
      this.selectedLayer.shape.gap = value;
    } else if ((propName === 'line.arm_count' || propName === 'shape.arm_count') && this.selectedLayer.shape.armCount !== undefined) {
      this.selectedLayer.shape.armCount = value;
    } else if ((propName === 'dot.diameter' || propName === 'shape.diameter') && this.selectedLayer.shape.diameter !== undefined) {
      this.selectedLayer.shape.diameter = value;
    } else if (propName === 'shape.radius' && this.selectedLayer.shape.radius !== undefined) {
      this.selectedLayer.shape.radius = value;
    } else if (propName === 'shape.inner_radius' && this.selectedLayer.shape.innerRadius !== undefined) {
      this.selectedLayer.shape.innerRadius = value;
    } else if (propName === 'shape.points' && this.selectedLayer.shape.points !== undefined) {
      this.selectedLayer.shape.points = Math.round(value);
    } else if (propName === 'shape.angle' && this.selectedLayer.shape.angle !== undefined) {
      this.selectedLayer.shape.angle = value;
    } else if (propName === 'shape.width' && this.selectedLayer.shape.width !== undefined) {
      this.selectedLayer.shape.width = value;
    } else if (propName === 'shape.height' && this.selectedLayer.shape.height !== undefined) {
      this.selectedLayer.shape.height = value;
    } else if (propName === 'shape.corner_radius' && this.selectedLayer.shape.cornerRadius !== undefined) {
      this.selectedLayer.shape.cornerRadius = value;
    } else if (propName === 'shape.base' && this.selectedLayer.shape.base !== undefined) {
      this.selectedLayer.shape.base = value;
    } else if (propName === 'shape.start_angle' && this.selectedLayer.shape.startAngle !== undefined) {
      this.selectedLayer.shape.startAngle = value;
    } else if (propName === 'shape.end_angle' && this.selectedLayer.shape.endAngle !== undefined) {
      this.selectedLayer.shape.endAngle = value;
    } else if (propName === 'shape.center_hole' && this.selectedLayer.shape.centerHole !== undefined) {
      this.selectedLayer.shape.centerHole = value;
    } else if (propName === 'effect.shadow_blur') {
      this.selectedLayer.shadowBlur = value;
    } else if (propName === 'effect.shadow_offset_x') {
      this.selectedLayer.shadowOffsetX = value;
    } else if (propName === 'effect.shadow_offset_y') {
      this.selectedLayer.shadowOffsetY = value;
    } else if (propName === 'effect.blur_radius') {
      this.selectedLayer.blurRadius = value;
    } else if (propName === 'effect.glow_radius') {
      this.selectedLayer.glowRadius = value;
    } else if (propName === 'layer.offset_x' || propName === 'transform.offset_x') {
      this.selectedLayer.offsetX = value;
    } else if (propName === 'layer.offset_y' || propName === 'transform.offset_y') {
      this.selectedLayer.offsetY = value;
    } else if (propName === 'layer.rotation' || propName === 'transform.rotation') {
      this.selectedLayer.rotation = value;
    } else if (propName === 'global.opacity' || propName === 'layer.opacity') {
      this.selectedLayer.opacity = value;
    }
  }

  setLayerColor(color: string) {
    if (!this.selectedLayer) return;
    this.pushUndo();
    this.selectedLayer.color = color;
  }

  setShapePathData(pathData: string) {
    if (!this.selectedLayer) return;
    this.pushUndo();
    this.selectedLayer.shape.pathData = pathData;
  }

  setDotSquare(isSquare: boolean) {
    if (!this.selectedLayer) return;
    this.pushUndo();
    this.selectedLayer.shape.isSquare = isSquare;
  }

  setLayerGlow(radius: number, color?: string) {
    if (!this.selectedLayer) return;
    this.pushUndo();
    this.selectedLayer.glowRadius = radius;
    if (color) {
      this.selectedLayer.glowColor = color;
    } else if (!this.selectedLayer.glowColor) {
      this.selectedLayer.glowColor = this.selectedLayer.color;
    }
  }

  setPreviewBackground(bg: PreviewBackground) {
    this.previewBackground = bg;
  }

  setLayerShadow(color: string, blur: number, offsetX: number, offsetY: number) {
    if (!this.selectedLayer) return;
    this.pushUndo();
    this.selectedLayer.shadowColor = color;
    this.selectedLayer.shadowBlur = blur;
    this.selectedLayer.shadowOffsetX = offsetX;
    this.selectedLayer.shadowOffsetY = offsetY;
  }

  setLayerBlur(radius: number) {
    if (!this.selectedLayer) return;
    this.pushUndo();
    this.selectedLayer.blurRadius = radius;
  }

  // Import / Export (§11, §17.4 Screen 16)
  openImportExport(tab: 'import' | 'export' = 'import') {
    this.importExportTab = tab;
    this.importExportModalOpen = true;
  }

  closeImportExport() {
    this.importExportModalOpen = false;
  }

  exportCrosshairJson(): string {
    return JSON.stringify(this.crosshair, null, 2);
  }

  exportCrosshairCode(): string {
    try {
      const payload = JSON.stringify(this.crosshair);
      return `XH1-${btoa(payload)}`;
    } catch {
      return `XH1-${encodeURIComponent(JSON.stringify(this.crosshair))}`;
    }
  }

  importCrosshair(input: string): { success: boolean; message: string } {
    const trimmed = input.trim();
    if (!trimmed) {
      return { success: false, message: 'Input is empty' };
    }

    this.pushUndo();

    // 1. JSON format (.xhair)
    if (trimmed.startsWith('{') && trimmed.endsWith('}')) {
      try {
        const parsed = JSON.parse(trimmed);
        if (parsed.layers && Array.isArray(parsed.layers)) {
          const imported: Crosshair = {
            id: parsed.id || `ch-${Date.now()}`,
            name: parsed.name || 'Imported Crosshair',
            description: parsed.description || 'Imported from .xhair file',
            author: parsed.author || 'Imported',
            tags: parsed.tags || ['Imported'],
            favorite: Boolean(parsed.favorite),
            formatVersion: parsed.formatVersion || 1,
            offsetX: parsed.offsetX ?? 0,
            offsetY: parsed.offsetY ?? 0,
            scale: parsed.scale ?? 1.0,
            opacity: parsed.opacity ?? 1.0,
            layers: parsed.layers,
            animation: parsed.animation,
          };
          this.library.push(imported);
          this.crosshair = JSON.parse(JSON.stringify(imported));
          this.selectedLayerId = imported.layers[0]?.id || '';
          return { success: true, message: `Successfully imported "${imported.name}" (.xhair format)` };
        }
      } catch (err: unknown) {
        return { success: false, message: `Invalid JSON syntax: ${err instanceof Error ? err.message : String(err)}` };
      }
    }

    // 2. Share Code (XH1-...)
    if (trimmed.startsWith('XH1-')) {
      const code = trimmed.slice(4);
      try {
        const decoded = atob(code);
        const parsed = JSON.parse(decoded);
        if (parsed.layers && Array.isArray(parsed.layers)) {
          const imported: Crosshair = {
            id: uniqueId('ch'),
            name: parsed.name ? `${parsed.name} (Shared)` : 'Imported Share Code',
            description: parsed.description || 'Imported via XH1 share code',
            author: parsed.author || 'Shared',
            tags: parsed.tags || ['ShareCode'],
            favorite: false,
            formatVersion: 1,
            offsetX: parsed.offsetX ?? 0,
            offsetY: parsed.offsetY ?? 0,
            scale: parsed.scale ?? 1.0,
            opacity: parsed.opacity ?? 1.0,
            layers: parsed.layers,
          };
          this.library.push(imported);
          this.crosshair = JSON.parse(JSON.stringify(imported));
          this.selectedLayerId = imported.layers[0]?.id || '';
          return { success: true, message: `Successfully imported "${imported.name}" from share code` };
        }
      } catch {
        // Fallback
      }
    }

    // 3. Valorant In-Game Crosshair Code (e.g. 0;P;c;5;h;0;0l;4;0v;2;0g;1;0a;1)
    if (trimmed.startsWith('0;') || trimmed.includes(';0l;') || trimmed.includes(';0t;')) {
      try {
        const tokens = trimmed.split(';');
        const map: Record<string, string> = {};
        for (let i = 0; i < tokens.length; i += 2) {
          if (i + 1 < tokens.length) {
            map[tokens[i]] = tokens[i + 1];
          }
        }

        const colorMap: Record<string, string> = {
          '0': '#FFFFFF',
          '1': '#00FF00',
          '2': '#7FFF00',
          '3': '#DFFF00',
          '4': '#FFFF00',
          '5': '#00FFFF',
          '6': '#FF1493',
          '7': '#FF0000',
        };
        const color = map['u'] ? `#${map['u'].replace('#', '')}` : (colorMap[map['c'] || '5'] || '#00FFFF');
        const len = parseFloat(map['0l'] || '4') || 4;
        const thick = parseFloat(map['0v'] || map['0t'] || '2') || 2;
        const gap = parseFloat(map['0g'] || map['0o'] || '2') || 2;
        const opacity = parseFloat(map['0a'] || '1') || 1;
        const hasDot = map['d'] === '1';
        const dotSize = parseFloat(map['z'] || '2') || 2;

        const layers: Layer[] = [
          {
            id: uniqueId('val-arms'),
            name: 'Inner Lines',
            visible: true,
            locked: false,
            shape: { type: 'LineArms', length: len, thickness: thick, gap, armCount: 4 },
            color,
            opacity,
            offsetX: 0,
            offsetY: 0,
            rotation: 0,
          },
        ];

        if (hasDot) {
          layers.push({
            id: uniqueId('val-dot'),
            name: 'Center Dot',
            visible: true,
            locked: false,
            shape: { type: 'Dot', diameter: dotSize, isSquare: true },
            color,
            opacity: 1.0,
            offsetX: 0,
            offsetY: 0,
            rotation: 0,
          });
        }

        const imported: Crosshair = {
          id: uniqueId('ch-val'),
          name: 'Valorant Profile',
          description: `Imported from Valorant code: ${trimmed.slice(0, 24)}...`,
          author: 'Valorant',
          tags: ['Valorant', 'Import'],
          favorite: false,
          formatVersion: 1,
          offsetX: 0,
          offsetY: 0,
          scale: 1.0,
          opacity: 1.0,
          layers,
        };
        this.library.push(imported);
        this.crosshair = JSON.parse(JSON.stringify(imported));
        this.selectedLayerId = imported.layers[0]?.id || '';
        return { success: true, message: 'Successfully parsed and imported Valorant crosshair code' };
      } catch (err: unknown) {
        return { success: false, message: `Failed to parse Valorant crosshair code: ${err instanceof Error ? err.message : String(err)}` };
      }
    }

    return { success: false, message: 'Unrecognized format. Expected .xhair JSON, XH1- share code, or Valorant code' };
  }

  // Resolution Management (§6.3, §17.4 Screen 14)
  openResolutionModal() {
    this.resolutionModalOpen = true;
  }

  closeResolutionModal() {
    this.resolutionModalOpen = false;
  }

  selectResolutionProfile(profile: ResolutionProfile) {
    this.activeProfile = profile;
  }

  addCustomResolutionProfile(name: string, width: number, height: number, isStretched: boolean) {
    const id = uniqueId(`custom-${width}x${height}`);
    const newProfile: ResolutionProfile = {
      id,
      name: `${name} (${width}×${height}${isStretched ? ' Stretched' : ''})`,
      width: Math.max(320, Math.round(width)),
      height: Math.max(240, Math.round(height)),
      isStretched,
      isNative: false,
      dpiScale: 1.0,
    };
    RESOLUTION_PROFILES.push(newProfile);
    this.activeProfile = newProfile;
  }

  // Calibration Methods (§6.7, M8)
  openCalibration() {
    this.calibrationOpen = true;
    this.calibrationStep = 1;
    this.calibrationOffset = { ...(this.activeProfile.calibrationOffset || { x: 0, y: 0 }) };
  }

  closeCalibration() {
    this.calibrationOpen = false;
    this.calibrationPatternActive = false;
  }

  setCalibrationStep(step: number) {
    this.calibrationStep = Math.min(4, Math.max(1, step));
    if (this.calibrationStep >= 2) {
      this.calibrationPatternActive = true;
    }
  }

  toggleCalibrationPattern() {
    this.calibrationPatternActive = !this.calibrationPatternActive;
  }

  nudgeCalibration(dx: number, dy: number) {
    this.calibrationOffset.x = Math.round((this.calibrationOffset.x + dx) * 4) / 4;
    this.calibrationOffset.y = Math.round((this.calibrationOffset.y + dy) * 4) / 4;
  }

  saveCalibrationOffset() {
    this.activeProfile.calibrationOffset = { ...this.calibrationOffset };
  }

  resetCalibrationOffset() {
    this.calibrationOffset = { x: 0, y: 0 };
    if (this.activeProfile.calibrationOffset) {
      this.activeProfile.calibrationOffset = { x: 0, y: 0 };
    }
  }

  setScreenshotAlignResult(offset: { x: number; y: number }, confidence: number) {
    this.screenshotDetectedOffset = offset;
    this.screenshotConfidence = confidence;
  }

  applyScreenshotAlignOffset() {
    if (this.screenshotDetectedOffset) {
      this.calibrationOffset = { ...this.screenshotDetectedOffset };
      this.saveCalibrationOffset();
    }
  }

  apply1ClickAlignmentFix(issue: AlignmentIssue) {
    const layer = this.crosshair.layers.find((l) => l.id === issue.layer_id);
    if (!layer) return;
    this.pushUndo();

    if (issue.suggested_fix.type === 'adjust_thickness' && issue.suggested_fix.new_thickness !== undefined) {
      if (layer.shape.type === 'LineArms') {
        layer.shape.thickness = issue.suggested_fix.new_thickness;
      } else if (layer.shape.type === 'Dot') {
        layer.shape.diameter = issue.suggested_fix.new_thickness;
      }
    } else if (issue.suggested_fix.type === 'offset_half_pixel' && issue.suggested_fix.delta !== undefined) {
      if (issue.axis === 'horizontal' || issue.axis === 'both') {
        layer.offsetX += issue.suggested_fix.delta;
      }
      if (issue.axis === 'vertical' || issue.axis === 'both') {
        layer.offsetY += issue.suggested_fix.delta;
      }
    }
  }

  openSprayLab() {
    this.sprayLabOpen = true;
  }

  closeSprayLab() {
    this.sprayLabOpen = false;
  }

  requestDynamicOptIn() {
    this.dynamicOptInOpen = true;
  }

  confirmDynamicOptIn() {
    this.dynamicReticleEnabled = true;
    this.dynamicOptInOpen = false;
  }

  disableDynamicReticle() {
    this.dynamicReticleEnabled = false;
  }

  openCompliancePanel() {
    this.compliancePanelOpen = true;
  }

  closeCompliancePanel() {
    this.compliancePanelOpen = false;
  }

  addDynamicBinding(binding: DynamicBinding) {
    this.pushUndo();
    this.dynamicBindings = [...this.dynamicBindings, binding];
  }

  removeDynamicBinding(bindingId: string) {
    this.pushUndo();
    this.dynamicBindings = this.dynamicBindings.filter((b) => b.id !== bindingId);
  }
}

export const editorState = new EditorState();
