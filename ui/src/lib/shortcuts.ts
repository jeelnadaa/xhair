export interface CommandShortcut {
  id: string;
  name: string;
  category: 'Edit' | 'Tools' | 'View' | 'Nudge' | 'Overlay' | 'System';
  shortcut: string;
  description: string;
  action: string;
}

export const SHORTCUTS_REGISTRY: CommandShortcut[] = [
  // Edit
  {
    id: 'edit.undo',
    name: 'Undo',
    category: 'Edit',
    shortcut: 'Ctrl+Z',
    description: 'Undo the last action or command step',
    action: 'undo',
  },
  {
    id: 'edit.redo',
    name: 'Redo',
    category: 'Edit',
    shortcut: 'Ctrl+Y',
    description: 'Redo the previously undone action',
    action: 'redo',
  },
  {
    id: 'edit.duplicate',
    name: 'Duplicate Layer',
    category: 'Edit',
    shortcut: 'Ctrl+D',
    description: 'Clone the currently selected layer',
    action: 'duplicateLayer',
  },
  {
    id: 'edit.delete',
    name: 'Delete Layer',
    category: 'Edit',
    shortcut: 'Delete',
    description: 'Remove the currently selected layer',
    action: 'deleteLayer',
  },
  {
    id: 'edit.command_palette',
    name: 'Command Palette',
    category: 'Edit',
    shortcut: 'Ctrl+K',
    description: 'Open the searchable spotlight command palette',
    action: 'openCommandPalette',
  },

  // Tools
  {
    id: 'tool.select',
    name: 'Select Tool',
    category: 'Tools',
    shortcut: 'V',
    description: 'Select, move, and transform layers',
    action: 'selectTool:select',
  },
  {
    id: 'tool.direct',
    name: 'Direct Select Tool',
    category: 'Tools',
    shortcut: 'A',
    description: 'Sub-select individual arms, points, and anchors',
    action: 'selectTool:direct',
  },
  {
    id: 'tool.lines',
    name: 'Line Arms Tool',
    category: 'Tools',
    shortcut: 'L',
    description: 'Draw standard 4-way or radial crosshair arms',
    action: 'selectTool:lines',
  },
  {
    id: 'tool.dot',
    name: 'Dot Tool',
    category: 'Tools',
    shortcut: 'D',
    description: 'Create a center reticle dot (square or round)',
    action: 'selectTool:dot',
  },
  {
    id: 'tool.rect',
    name: 'Rect Tool',
    category: 'Tools',
    shortcut: 'R',
    description: 'Create a rectangle / box reticle element',
    action: 'selectTool:rect',
  },
  {
    id: 'tool.circle',
    name: 'Circle / Ring Tool',
    category: 'Tools',
    shortcut: 'C',
    description: 'Create a circular ring or hollow reticle',
    action: 'selectTool:circle',
  },
  {
    id: 'tool.pen',
    name: 'Path / Pen Tool',
    category: 'Tools',
    shortcut: 'P',
    description: 'Plot custom polygonal or bezier vector shapes',
    action: 'selectTool:pen',
  },
  {
    id: 'tool.brush',
    name: 'Pixel Brush Tool',
    category: 'Tools',
    shortcut: 'B',
    description: 'Direct bitmap pixel grid painting',
    action: 'selectTool:brush',
  },

  // View
  {
    id: 'view.zoom_in',
    name: 'Zoom In',
    category: 'View',
    shortcut: 'Ctrl++',
    description: 'Increase canvas magnification factor',
    action: 'zoomIn',
  },
  {
    id: 'view.zoom_out',
    name: 'Zoom Out',
    category: 'View',
    shortcut: 'Ctrl+-',
    description: 'Decrease canvas magnification factor',
    action: 'zoomOut',
  },
  {
    id: 'view.zoom_reset',
    name: 'Zoom 1:1 (True Size)',
    category: 'View',
    shortcut: 'Ctrl+0',
    description: 'Reset zoom to exact 1:1 physical monitor scale',
    action: 'zoomReset',
  },
  {
    id: 'view.toggle_grid',
    name: 'Toggle Grid',
    category: 'View',
    shortcut: 'G',
    description: 'Toggle pixel grid and major alignment subdivisions',
    action: 'toggleGrid',
  },
  {
    id: 'view.toggle_rulers',
    name: 'Toggle Rulers',
    category: 'View',
    shortcut: 'Shift+R',
    description: 'Show or hide pixel distance rulers and coordinate headers',
    action: 'toggleRulers',
  },
  {
    id: 'view.toggle_guides',
    name: 'Toggle Center Guides',
    category: 'View',
    shortcut: ';',
    description: 'Show or hide center crosshair guides and distance rings',
    action: 'toggleGuides',
  },

  // Nudge
  {
    id: 'nudge.up',
    name: 'Nudge Up',
    category: 'Nudge',
    shortcut: 'ArrowUp',
    description: 'Nudge the active selection up by the current step',
    action: 'nudge:up',
  },
  {
    id: 'nudge.down',
    name: 'Nudge Down',
    category: 'Nudge',
    shortcut: 'ArrowDown',
    description: 'Nudge the active selection down by the current step',
    action: 'nudge:down',
  },
  {
    id: 'nudge.left',
    name: 'Nudge Left',
    category: 'Nudge',
    shortcut: 'ArrowLeft',
    description: 'Nudge the active selection left by the current step',
    action: 'nudge:left',
  },
  {
    id: 'nudge.right',
    name: 'Nudge Right',
    category: 'Nudge',
    shortcut: 'ArrowRight',
    description: 'Nudge the active selection right by the current step',
    action: 'nudge:right',
  },
  {
    id: 'nudge.all_up',
    name: 'Nudge Crosshair Up',
    category: 'Nudge',
    shortcut: 'Ctrl+ArrowUp',
    description: 'Nudge the entire crosshair up regardless of selection',
    action: 'nudgeAll:up',
  },

  // Overlay
  {
    id: 'overlay.toggle',
    name: 'Toggle Overlay',
    category: 'Overlay',
    shortcut: 'F10',
    description: 'Toggle desktop layered crosshair visibility',
    action: 'toggleOverlay',
  },
  {
    id: 'overlay.calibrate',
    name: 'Open Calibration',
    category: 'Overlay',
    shortcut: 'F11',
    description: 'Launch the interactive 4-step pixel calibration wizard',
    action: 'openCalibration',
  },
  {
    id: 'calibration.toggle_pattern',
    name: 'Toggle Calibration Pattern',
    category: 'Overlay',
    shortcut: 'F8',
    description: 'Toggle calibration test pattern flicker comparison (§6.7)',
    action: 'toggleCalibrationPattern',
  },
];

export function getShortcutForAction(action: string): string | undefined {
  const found = SHORTCUTS_REGISTRY.find((s) => s.action === action || s.id === action);
  return found?.shortcut;
}

export function getCommandById(id: string): CommandShortcut | undefined {
  return SHORTCUTS_REGISTRY.find((s) => s.id === id);
}
