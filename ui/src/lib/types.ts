export type ShapeType =
  | 'Dot'
  | 'LineArms'
  | 'Cross'
  | 'Circle'
  | 'Ring'
  | 'Arc'
  | 'Rect'
  | 'Triangle'
  | 'Polygon'
  | 'Star'
  | 'Chevron'
  | 'TShape'
  | 'Plus'
  | 'SvgPath'
  | 'Group';

export type BlendMode = 'Normal' | 'Screen' | 'Multiply' | 'Additive';

export type SymmetryType = 'None' | 'MirrorH' | 'MirrorV' | 'MirrorHV' | 'Radial';

export type PreviewBackground = 'dark' | 'light' | 'foliage' | 'desert' | 'urban';

export interface ShapeData {
  type: ShapeType;
  // Line arms / TShape / Plus
  length?: number;
  thickness?: number;
  gap?: number;
  armCount?: number;
  armOffsets?: Array<{ x: number; y: number }>;
  centerHole?: number;
  // Dot
  diameter?: number;
  isSquare?: boolean;
  // Ring / Circle / Arc
  radius?: number;
  startAngle?: number;
  endAngle?: number;
  // Rect
  width?: number;
  height?: number;
  cornerRadius?: number;
  // Triangle
  base?: number;
  // Chevron / Polygon / Star
  innerOffset?: number;
  armThickness?: number;
  angle?: number;
  points?: number;
  innerRadius?: number;
  // Custom SVG path
  pathData?: string;
  // Group container
  children?: string[];
  isExpanded?: boolean;
}

export interface Layer {
  id: string;
  name: string;
  visible: boolean;
  locked: boolean;
  shape: ShapeData;
  color: string;
  opacity: number;
  offsetX: number;
  offsetY: number;
  rotation: number;
  groupId?: string;
  isGroup?: boolean;
  children?: string[];
  isExpanded?: boolean;
  glowRadius?: number;
  glowColor?: string;
  shadowColor?: string;
  shadowOffsetX?: number;
  shadowOffsetY?: number;
  shadowBlur?: number;
  blurRadius?: number;
}

export interface CrosshairGroup {
  id: string;
  name: string;
  parent_id?: string;
  icon?: string;
  color?: string;
  collapsed?: boolean;
  order?: number;
  game_binding?: string;
  active_crosshair_id?: string;
  default_resolution_profile?: string;
}

export interface Crosshair {
  id: string;
  name: string;
  description: string;
  author: string;
  tags: string[];
  favorite: boolean;
  formatVersion: number;
  groupId?: string;
  offsetX: number;
  offsetY: number;
  scale: number;
  opacity: number;
  layers: Layer[];
  animation?: AnimTrack[];
}

export type AnimKindType = 'pulse' | 'breathe' | 'rotate' | 'blink' | 'rainbow_cycle' | 'keyframes';
export type EasingMode = 'linear' | 'ease_in' | 'ease_out' | 'ease_in_out' | 'step';

export interface Keyframe {
  time: number;
  value: number;
}

export interface AnimTrack {
  id: string;
  target_path: string;
  kind: AnimKindType;
  duration_seconds: number;
  speed?: number;
  min?: number;
  max?: number;
  frequency_hz?: number;
  keyframes?: Keyframe[];
}

export interface PropertyDescriptor {
  name: string;
  label: string;
  tip: string;
  unit: string | null;
  min: number;
  max: number;
  step: number;
  fine_step: number;
  default_value: number;
  group: string;
}

export interface ResolutionProfile {
  id: string;
  name: string;
  width: number;
  height: number;
  isStretched: boolean;
  isNative: boolean;
  dpiScale: number;
  calibrationOffset?: { x: number; y: number };
}

export type ToolType = 'select' | 'direct' | 'lines' | 'dot' | 'rect' | 'circle' | 'pen' | 'brush';

export type NudgeTarget = 'arm' | 'layer' | 'all';
export type NudgeFrame = 'screen' | 'local';

export interface AlignmentIssue {
  layer_id: string;
  layer_name: string;
  axis: 'horizontal' | 'vertical' | 'both';
  edge_position: number;
  suggested_fix: {
    type: 'offset_half_pixel' | 'adjust_thickness';
    delta?: number;
    new_thickness?: number;
  };
}

export interface AlignmentDiagnostic {
  isEvenWidth: boolean;
  isEvenHeight: boolean;
  centerEdgeOrMiddle: 'pixel-edge' | 'pixel-middle';
  message: string;
  hasSubpixelOffset: boolean;
  issues?: AlignmentIssue[];
}

export interface ScreenshotAlignResult {
  detected_center: [number, number];
  theoretical_center: [number, number];
  suggested_offset: [number, number];
  confidence: number;
}

export interface SpreadConfig {
  base: number;
  per_shot: number;
  max: number;
  recovery_per_sec: number;
  move_penalty: number;
  shift_move_penalty: number;
  crouch_bonus: number;
  air_penalty: number;
}

export interface SmoothingConfig {
  attack_ms: number;
  release_ms: number;
}

export interface GunProfile {
  id: string;
  name: string;
  notes: string;
  fire_rate_hz: number;
  magazine: number;
  spread: SpreadConfig;
  smoothing: SmoothingConfig;
  pattern: Array<{ x: number; y: number }>;
}

export type DriverSource = 'spread' | 'movement_speed' | 'firing_heat' | 'recoil_x' | 'recoil_y';
export type BindingCurve = 'linear' | 'ease_in' | 'ease_out' | 'ease_in_out';
export type BindingMode = 'add' | 'set' | 'multiply';

export interface DynamicBinding {
  id: string;
  property_path: string;
  driver: DriverSource;
  in_range: [number, number];
  out_range: [number, number];
  curve: BindingCurve;
  mode: BindingMode;
}

export interface SprayShotImpact {
  shot_index: number;
  position: { x: number; y: number };
  timestamp_seconds: number;
  spread_radius: number;
}
