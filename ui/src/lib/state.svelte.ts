import type {
  Crosshair,
  Layer,
  PropertyDescriptor,
  ResolutionProfile,
  ToolType,
  NudgeTarget,
  NudgeFrame,
  AlignmentDiagnostic,
  AnimTrack,
} from './types';
import propertiesData from '../generated/properties.json';

export const DESCRIPTORS: PropertyDescriptor[] = propertiesData.descriptors as PropertyDescriptor[];

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

  // Nudge Control
  nudgeStep = $state<number>(0.5);
  nudgeTarget = $state<NudgeTarget>('arm');
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

  // Command Palette & Modals
  commandPaletteOpen = $state<boolean>(false);

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

  // Calculate visibility score against dark base background (#0B0D10)
  visibilityScore = $derived.by<number>(() => {
    if (!this.selectedLayer) return 7.5;
    const hex = this.selectedLayer.color.replace('#', '');
    const r = parseInt(hex.substring(0, 2), 16) || 0;
    const g = parseInt(hex.substring(2, 4), 16) || 0;
    const b = parseInt(hex.substring(4, 6), 16) || 0;
    // Relative luminance calculation
    const lum = 0.2126 * (r / 255) + 0.7152 * (g / 255) + 0.0722 * (b / 255);
    const bgLum = 0.005; // #0B0D10
    const ratio = (lum + 0.05) / (bgLum + 0.05);
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
    const newId = `ch-${Date.now()}`;
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
          id: `layer-${Date.now()}`,
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

  // Nudge in 8 directions (§9.4)
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
      // For arm nudge, adjust layer offset or gap based on arm direction
      if (this.selectedArmIndex === 0) {
        // Top arm: up decreases gap, down increases gap
        if (this.selectedLayer.shape.gap !== undefined) {
          this.selectedLayer.shape.gap = Math.max(0, this.selectedLayer.shape.gap - stepY);
        }
      } else if (this.selectedArmIndex === 1) {
        // Right arm
        if (this.selectedLayer.shape.gap !== undefined) {
          this.selectedLayer.shape.gap = Math.max(0, this.selectedLayer.shape.gap + stepX);
        }
      } else if (this.selectedArmIndex === 2) {
        // Bottom arm
        if (this.selectedLayer.shape.gap !== undefined) {
          this.selectedLayer.shape.gap = Math.max(0, this.selectedLayer.shape.gap + stepY);
        }
      } else if (this.selectedArmIndex === 3) {
        // Left arm
        if (this.selectedLayer.shape.gap !== undefined) {
          this.selectedLayer.shape.gap = Math.max(0, this.selectedLayer.shape.gap - stepX);
        }
      }
    }
  }

  resetNudge() {
    this.pushUndo();
    if (this.nudgeTarget === 'all') {
      this.crosshair.offsetX = 0;
      this.crosshair.offsetY = 0;
    } else if (this.selectedLayer) {
      this.selectedLayer.offsetX = 0;
      this.selectedLayer.offsetY = 0;
    }
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
      cloned.id = `layer-${Date.now()}`;
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

    if (propName === 'line.length' && this.selectedLayer.shape.length !== undefined) {
      this.selectedLayer.shape.length = value;
    } else if (propName === 'line.thickness' && this.selectedLayer.shape.thickness !== undefined) {
      this.selectedLayer.shape.thickness = value;
    } else if (propName === 'line.gap' && this.selectedLayer.shape.gap !== undefined) {
      this.selectedLayer.shape.gap = value;
    } else if (propName === 'line.arm_count' && this.selectedLayer.shape.armCount !== undefined) {
      this.selectedLayer.shape.armCount = value;
    } else if (propName === 'dot.diameter' && this.selectedLayer.shape.diameter !== undefined) {
      this.selectedLayer.shape.diameter = value;
    } else if (propName === 'global.opacity') {
      this.selectedLayer.opacity = value;
    }
  }

  setLayerColor(color: string) {
    if (!this.selectedLayer) return;
    this.pushUndo();
    this.selectedLayer.color = color;
  }
}

export const editorState = new EditorState();
