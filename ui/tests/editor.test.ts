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
});
