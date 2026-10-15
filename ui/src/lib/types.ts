export type ShapeType =
  | 'Dot'
  | 'LineArms'
  | 'Cross'
  | 'Circle'
  | 'Ring'
  | 'Arc'
  | 'Polygon'
  | 'Star'
  | 'Chevron'
  | 'TShape'
  | 'Plus'
  | 'SvgPath';

export type BlendMode = 'Normal' | 'Screen' | 'Multiply' | 'Additive';

export type SymmetryType = 'None' | 'MirrorH' | 'MirrorV' | 'MirrorHV' | 'Radial';

export interface ShapeData {
  type: ShapeType;
  // Line arms
  length?: number;
  thickness?: number;
  gap?: number;
  armCount?: number;
  // Dot
  diameter?: number;
  isSquare?: boolean;
  // Ring / Circle
  radius?: number;
  // Cross / Plus / T-Shape
  innerOffset?: number;
  armThickness?: number;
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
  glowRadius?: number;
  glowColor?: string;
}

export interface Crosshair {
  id: string;
  name: string;
  description: string;
  author: string;
  tags: string[];
  favorite: boolean;
  formatVersion: number;
  offsetX: number;
  offsetY: number;
  scale: number;
  opacity: number;
  layers: Layer[];
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
}

export type ToolType = 'select' | 'direct' | 'lines' | 'dot' | 'rect' | 'circle' | 'pen' | 'brush';

export type NudgeTarget = 'arm' | 'layer' | 'all';
export type NudgeFrame = 'screen' | 'local';

export interface AlignmentDiagnostic {
  isEvenWidth: boolean;
  isEvenHeight: boolean;
  centerEdgeOrMiddle: 'pixel-edge' | 'pixel-middle';
  message: string;
  hasSubpixelOffset: boolean;
}
