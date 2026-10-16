import { describe, it, expect, beforeEach } from 'vitest';
import { EditorState, DESCRIPTORS, RESOLUTION_PROFILES } from '../src/lib/state.svelte';
import { SHORTCUTS_REGISTRY } from '../src/lib/shortcuts';

describe('Editor State & Logic', () => {
  let state: EditorState;

  beforeEach(() => {
    state = new EditorState();
  });

  it('loads property descriptors from generated JSON', () => {
    expect(DESCRIPTORS.length).toBeGreaterThan(0);
    const lengthDesc = DESCRIPTORS.find((d) => d.name === 'line.length');
    expect(lengthDesc).toBeDefined();
    expect(lengthDesc?.label).toBe('Length');
    expect(lengthDesc?.unit).toBe('px');
  });

  it('initializes with default crosshair and layers', () => {
    expect(state.crosshair.name).toBe('Cyan Gap');
    expect(state.crosshair.layers.length).toBe(3);
    expect(state.selectedLayer).toBeDefined();
    expect(state.selectedLayer?.name).toBe('Arms ×4');
  });

  it('handles fine nudging with target=all', () => {
    state.nudgeTarget = 'all';
    state.nudgeStep = 0.5;

    // Nudge right (dx=1, dy=0)
    state.nudge(1, 0);
    expect(state.crosshair.offsetX).toBe(0.5);
    expect(state.crosshair.offsetY).toBe(0);

    // Nudge up-left (dx=-1, dy=-1)
    state.nudge(-1, -1);
    expect(state.crosshair.offsetX).toBe(0);
    expect(state.crosshair.offsetY).toBe(-0.5);

    // Reset
    state.resetNudge();
    expect(state.crosshair.offsetX).toBe(0);
    expect(state.crosshair.offsetY).toBe(0);
  });

  it('handles fine nudging with target=layer', () => {
    state.nudgeTarget = 'layer';
    state.nudgeStep = 0.25;

    state.nudge(1, 1);
    expect(state.selectedLayer.offsetX).toBe(0.25);
    expect(state.selectedLayer.offsetY).toBe(0.25);
  });

  it('handles fine nudging with target=arm', () => {
    state.nudgeTarget = 'arm';
    state.selectedArmIndex = 1; // Right arm
    state.nudgeStep = 1.0;

    state.nudge(1, 0);
    expect(state.selectedLayer.shape.armOffsets?.[1]?.x).toBe(1.0);
  });

  it('supports undo and redo', () => {
    state.nudgeTarget = 'all';
    state.nudgeStep = 2.0;

    state.nudge(1, 0);
    expect(state.crosshair.offsetX).toBe(2.0);

    state.undo();
    expect(state.crosshair.offsetX).toBe(0);

    state.redo();
    expect(state.crosshair.offsetX).toBe(2.0);
  });

  it('manages layer lifecycle: duplicate and delete', () => {
    const initialCount = state.crosshair.layers.length;
    state.duplicateLayer(state.selectedLayerId);
    expect(state.crosshair.layers.length).toBe(initialCount + 1);

    const duplicatedId = state.selectedLayerId;
    state.deleteLayer(duplicatedId);
    expect(state.crosshair.layers.length).toBe(initialCount);
  });

  it('toggles layer visibility and lock states', () => {
    const layerId = state.selectedLayerId;
    expect(state.selectedLayer.visible).toBe(true);
    state.toggleLayerVisibility(layerId);
    expect(state.selectedLayer.visible).toBe(false);

    expect(state.selectedLayer.locked).toBe(false);
    state.toggleLayerLock(layerId);
    expect(state.selectedLayer.locked).toBe(true);
  });

  it('computes alignment diagnostics correctly for even vs odd resolutions', () => {
    // 1920x1080 is even x even -> pixel edge
    state.activeProfile = RESOLUTION_PROFILES[0]; // 1920x1080
    expect(state.alignmentDiagnostic.isEvenWidth).toBe(true);
    expect(state.alignmentDiagnostic.centerEdgeOrMiddle).toBe('pixel-edge');

    // Custom odd resolution profile
    state.activeProfile = {
      id: 'odd-res',
      name: '1921×1081',
      width: 1921,
      height: 1081,
      isStretched: false,
      isNative: false,
      dpiScale: 1.0,
    };
    expect(state.alignmentDiagnostic.isEvenWidth).toBe(false);
    expect(state.alignmentDiagnostic.centerEdgeOrMiddle).toBe('pixel-middle');
  });

  it('calculates WCAG contrast score', () => {
    const score = state.visibilityScore;
    expect(score).toBeGreaterThan(1);
    expect(typeof score).toBe('number');
  });

  it('ensures all shortcuts in registry have valid actions and descriptions', () => {
    expect(SHORTCUTS_REGISTRY.length).toBeGreaterThan(10);
    for (const item of SHORTCUTS_REGISTRY) {
      expect(item.id).toBeTruthy();
      expect(item.name).toBeTruthy();
      expect(item.description).toBeTruthy();
      expect(item.action).toBeTruthy();
    }
  });

  it('handles 4-step calibration wizard flow and offset saving (§6.7)', () => {
    expect(state.calibrationOpen).toBe(false);
    state.openCalibration();
    expect(state.calibrationOpen).toBe(true);
    expect(state.calibrationStep).toBe(1);

    // Step 2 turns on test pattern
    state.setCalibrationStep(2);
    expect(state.calibrationStep).toBe(2);
    expect(state.calibrationPatternActive).toBe(true);

    // Fine nudge by quarter pixels
    state.nudgeCalibration(0.25, -0.5);
    expect(state.calibrationOffset.x).toBe(0.25);
    expect(state.calibrationOffset.y).toBe(-0.5);

    // Save offset
    state.saveCalibrationOffset();
    expect(state.activeProfile.calibrationOffset?.x).toBe(0.25);
    expect(state.activeProfile.calibrationOffset?.y).toBe(-0.5);

    state.closeCalibration();
    expect(state.calibrationOpen).toBe(false);
  });

  it('detects alignment issues and applies 1-click automatic fix (§6.2)', () => {
    state.activeProfile = RESOLUTION_PROFILES[0]; // 1920x1080
    // Set layer thickness to 1.0 (1920+1 is odd -> off-grid)
    const layer = state.crosshair.layers[0];
    layer.shape.type = 'LineArms';
    layer.shape.thickness = 1.0;

    const issues = state.alignmentIssues;
    expect(issues.length).toBeGreaterThan(0);
    const targetIssue = issues.find((i) => i.layer_id === layer.id);
    expect(targetIssue).toBeDefined();

    if (targetIssue) {
      state.apply1ClickAlignmentFix(targetIssue);
      expect(layer.shape.thickness).toBe(2.0); // Fixed!
    }
  });

  it('manages dynamic reticle session opt-in and risk lifecycle (§2, §12)', () => {
    expect(state.dynamicReticleEnabled).toBe(false);
    expect(state.dynamicOptInOpen).toBe(false);

    // Request opt-in
    state.requestDynamicOptIn();
    expect(state.dynamicOptInOpen).toBe(true);
    expect(state.dynamicReticleEnabled).toBe(false);

    // Confirm opt-in
    state.confirmDynamicOptIn();
    expect(state.dynamicReticleEnabled).toBe(true);
    expect(state.dynamicOptInOpen).toBe(false);

    // Disable
    state.disableDynamicReticle();
    expect(state.dynamicReticleEnabled).toBe(false);
  });

  it('manages Spray Lab window and gun profile catalog (§12.6)', () => {
    expect(state.sprayLabOpen).toBe(false);
    state.openSprayLab();
    expect(state.sprayLabOpen).toBe(true);

    expect(state.gunProfiles.length).toBeGreaterThanOrEqual(3);
    const rifle = state.gunProfiles.find((g) => g.id === 'gun-rifle-01');
    expect(rifle).toBeDefined();
    expect(rifle?.magazine).toBe(30);

    state.closeSprayLab();
    expect(state.sprayLabOpen).toBe(false);
  });

  it('manages Compliance Transparency panel (§2)', () => {
    expect(state.compliancePanelOpen).toBe(false);
    state.openCompliancePanel();
    expect(state.compliancePanelOpen).toBe(true);
    state.closeCompliancePanel();
    expect(state.compliancePanelOpen).toBe(false);
  });

  it('adds and removes dynamic property bindings (§12.5)', () => {
    const initialBindings = state.dynamicBindings.length;
    const testBinding = {
      id: 'test-scale-spread',
      property_path: 'global.scale',
      driver: 'spread' as const,
      in_range: [1.0, 10.0] as [number, number],
      out_range: [1.0, 1.5] as [number, number],
      curve: 'linear' as const,
      mode: 'set' as const,
    };

    state.addDynamicBinding(testBinding);
    expect(state.dynamicBindings.length).toBe(initialBindings + 1);
    expect(state.dynamicBindings.some((b) => b.id === 'test-scale-spread')).toBe(true);

    state.removeDynamicBinding('test-scale-spread');
    expect(state.dynamicBindings.length).toBe(initialBindings);
  });

  it('adds and configures all shape primitives (Rect, Arc, Triangle, Plus)', () => {
    // 1. Rect
    state.addLayer('Rect');
    expect(state.selectedLayer.shape.type).toBe('Rect');
    state.updateProperty('shape.width', 24);
    state.updateProperty('shape.height', 16);
    state.updateProperty('shape.corner_radius', 4);
    expect(state.selectedLayer.shape.width).toBe(24);
    expect(state.selectedLayer.shape.height).toBe(16);
    expect(state.selectedLayer.shape.cornerRadius).toBe(4);

    // 2. Arc
    state.addLayer('Arc');
    expect(state.selectedLayer.shape.type).toBe('Arc');
    state.updateProperty('shape.radius', 14);
    state.updateProperty('shape.thickness', 3);
    state.updateProperty('shape.start_angle', 45);
    state.updateProperty('shape.end_angle', 225);
    expect(state.selectedLayer.shape.radius).toBe(14);
    expect(state.selectedLayer.shape.thickness).toBe(3);
    expect(state.selectedLayer.shape.startAngle).toBe(45);
    expect(state.selectedLayer.shape.endAngle).toBe(225);

    // 3. Triangle
    state.addLayer('Triangle');
    expect(state.selectedLayer.shape.type).toBe('Triangle');
    state.updateProperty('shape.base', 18);
    state.updateProperty('shape.height', 20);
    expect(state.selectedLayer.shape.base).toBe(18);
    expect(state.selectedLayer.shape.height).toBe(20);

    // 4. Plus
    state.addLayer('Plus');
    expect(state.selectedLayer.shape.type).toBe('Plus');
    state.updateProperty('shape.length', 8);
    state.updateProperty('shape.thickness', 2);
    state.updateProperty('shape.gap', 3);
    state.updateProperty('shape.center_hole', 4);
    expect(state.selectedLayer.shape.length).toBe(8);
    expect(state.selectedLayer.shape.thickness).toBe(2);
    expect(state.selectedLayer.shape.gap).toBe(3);
    expect(state.selectedLayer.shape.centerHole).toBe(4);
  });

  it('configures visual effects: glow, drop shadow, and blur (§4.6)', () => {
    state.setLayerGlow(6, '#36F2CB');
    expect(state.selectedLayer.glowRadius).toBe(6);
    expect(state.selectedLayer.glowColor).toBe('#36F2CB');

    state.setLayerShadow('#000000', 4, 1, 2);
    expect(state.selectedLayer.shadowColor).toBe('#000000');
    expect(state.selectedLayer.shadowBlur).toBe(4);
    expect(state.selectedLayer.shadowOffsetX).toBe(1);
    expect(state.selectedLayer.shadowOffsetY).toBe(2);

    state.setLayerBlur(1.5);
    expect(state.selectedLayer.blurRadius).toBe(1.5);
  });

  it('handles universal import and export (§11, §17.4 Screen 16)', () => {
    // 1. Export JSON and Share Code
    const jsonStr = state.exportCrosshairJson();
    expect(jsonStr).toContain('"name": "Cyan Gap"');
    const shareCode = state.exportCrosshairCode();
    expect(shareCode.startsWith('XH1-')).toBe(true);

    // 2. Import .xhair JSON
    const importRes1 = state.importCrosshair(jsonStr);
    expect(importRes1.success).toBe(true);
    expect(state.crosshair.name).toBe('Cyan Gap');

    // 3. Import XH1 Share Code
    const importRes2 = state.importCrosshair(shareCode);
    expect(importRes2.success).toBe(true);

    // 4. Import Valorant in-game code
    const valCode = '0;P;c;5;h;0;0l;4;0v;2;0g;1;0a;1;d;1;z;2';
    const importRes3 = state.importCrosshair(valCode);
    expect(importRes3.success).toBe(true);
    expect(state.crosshair.name).toBe('Valorant Profile');
    expect(state.crosshair.layers.length).toBe(2);
    expect(state.crosshair.layers[0].shape.type).toBe('LineArms');
    expect(state.crosshair.layers[1].shape.type).toBe('Dot');

    // 5. Error handling
    const errRes = state.importCrosshair('invalid gibberish');
    expect(errRes.success).toBe(false);
  });

  it('adds custom resolution profiles and manages stretch parity (§6.3)', () => {
    const initialCount = RESOLUTION_PROFILES.length;
    state.addCustomResolutionProfile('CS2 Stretch', 1440, 1080, true);
    expect(RESOLUTION_PROFILES.length).toBe(initialCount + 1);
    expect(state.activeProfile.width).toBe(1440);
    expect(state.activeProfile.height).toBe(1080);
    expect(state.activeProfile.isStretched).toBe(true);
  });

  it('calculates contrast against all 5 tactical backgrounds (§6.8)', () => {
    state.setPreviewBackground('dark');
    const scoreDark = state.visibilityScore;
    expect(scoreDark).toBeGreaterThan(1);

    state.setPreviewBackground('light');
    const scoreLight = state.visibilityScore;
    expect(scoreLight).toBeGreaterThan(1);

    state.setPreviewBackground('foliage');
    expect(state.visibilityScore).toBeGreaterThan(1);

    state.setPreviewBackground('desert');
    expect(state.visibilityScore).toBeGreaterThan(1);

    state.setPreviewBackground('urban');
    expect(state.visibilityScore).toBeGreaterThan(1);
  });

  it('saves current profile and updates library and localStorage (§10)', () => {
    state.crosshair.name = 'Custom Saved Crosshair';
    state.crosshair.layers[0].color = '#FF0055';
    expect(state.saveStatus).toBe('idle');

    state.saveCurrentProfile();
    expect(state.saveStatus).toBe('saved');

    const inLib = state.library.find((c) => c.id === state.crosshair.id);
    expect(inLib).toBeDefined();
    expect(inLib?.name).toBe('Custom Saved Crosshair');
    expect(inLib?.layers[0].color).toBe('#FF0055');

    // Rename crosshair
    state.renameCrosshair('Renamed Sniper Crosshair');
    expect(state.crosshair.name).toBe('Renamed Sniper Crosshair');
    const renamedInLib = state.library.find((c) => c.id === state.crosshair.id);
    expect(renamedInLib?.name).toBe('Renamed Sniper Crosshair');
  });

  it('duplicates and deletes crosshairs in library', () => {
    const originalCount = state.library.length;
    const initialActiveId = state.crosshair.id;

    state.duplicateCrosshair(initialActiveId);
    expect(state.library.length).toBe(originalCount + 1);
    expect(state.crosshair.name).toContain('(Copy)');
    const duplicatedId = state.crosshair.id;
    expect(duplicatedId).not.toBe(initialActiveId);

    // Delete the duplicated crosshair
    state.deleteCrosshair(duplicatedId);
    expect(state.library.length).toBe(originalCount);
    expect(state.library.find((c) => c.id === duplicatedId)).toBeUndefined();
  });

  it('handles expandable and collapsible studio rails', () => {
    expect(state.leftRailWidth).toBe(320);
    expect(state.rightRailWidth).toBe(340);
    expect(state.leftRailCollapsed).toBe(false);
    expect(state.rightRailCollapsed).toBe(false);

    // Toggle collapse
    state.toggleLeftRail();
    expect(state.leftRailCollapsed).toBe(true);
    state.toggleLeftRail();
    expect(state.leftRailCollapsed).toBe(false);

    state.toggleRightRail();
    expect(state.rightRailCollapsed).toBe(true);
    state.toggleRightRail();
    expect(state.rightRailCollapsed).toBe(false);

    // Resizing within bounds
    state.setLeftRailWidth(400);
    expect(state.leftRailWidth).toBe(400);

    // Clamp min / max
    state.setLeftRailWidth(100);
    expect(state.leftRailWidth).toBe(260); // Clamped to min 260
    state.setLeftRailWidth(800);
    expect(state.leftRailWidth).toBe(540); // Clamped to max 540

    state.setRightRailWidth(450);
    expect(state.rightRailWidth).toBe(450);
    state.setRightRailWidth(150);
    expect(state.rightRailWidth).toBe(280); // Clamped to min 280
    state.setRightRailWidth(900);
    expect(state.rightRailWidth).toBe(560); // Clamped to max 560
  });
});

