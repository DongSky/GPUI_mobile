//! Supported-component inventory and Material 3 parity status.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parity {
    Done,
    Partial,
    NotStarted,
}

impl Parity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Done => "done",
            Self::Partial => "partial",
            Self::NotStarted => "not started",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentEntry {
    pub name: &'static str,
    pub material: &'static str,
    pub docs: &'static str,
    pub parity: Parity,
    pub notes: &'static str,
}

pub const INVENTORY: &[ComponentEntry] = &[
    ComponentEntry {
        name: "Color scheme",
        material: "Color roles (baseline light/dark)",
        docs: "https://m3.material.io/styles/color/roles",
        parity: Parity::Done,
        notes: "androidx PaletteTokens / ColorLightTokens / ColorDarkTokens v0_210",
    },
    ComponentEntry {
        name: "Typography",
        material: "Type scale (15 baseline styles)",
        docs: "https://m3.material.io/styles/typography/type-scale-tokens",
        parity: Parity::Done,
        notes: "Roboto when installed else Liberation Sans + word-gap; 15 baseline + 15 emphasized (400→500 / 500→700); heroes use display/headline emphasized",
    },
    ComponentEntry {
        name: "Shape",
        material: "Shape scale",
        docs: "https://m3.material.io/styles/shape/shape-scale-tokens",
        parity: Parity::Done,
        notes: "Expressive scale: none … extra-extra-large (48) + full; large-increased 20",
    },
    ComponentEntry {
        name: "Elevation",
        material: "Elevation levels 0–5",
        docs: "https://m3.material.io/styles/elevation",
        parity: Parity::Done,
        notes: "dp levels + catalog shadow; no tonal-overlay GPU lighting",
    },
    ComponentEntry {
        name: "State layers",
        material: "Interaction states",
        docs: "https://m3.material.io/foundations/interaction/states/state-layers",
        parity: Parity::Done,
        notes: "hover 8% / focus 10% / pressed 10% / dragged 16% / disabled 12%+38%",
    },
    ComponentEntry {
        name: "Motion tokens",
        material: "Easing and duration",
        docs: "https://m3.material.io/styles/motion/easing-and-duration/tokens-specs",
        parity: Parity::Done,
        notes: "Expressive spatial/effects springs + legacy emphasized eval; catalog CSS morph; GPUI Animation clock on indeterminate/wavy progress",
    },
    ComponentEntry {
        name: "Button",
        material: "Common buttons (filled, tonal, elevated, outlined, text)",
        docs: "https://m3.material.io/components/buttons/specs",
        parity: Parity::Done,
        notes: "Expressive XS–XL, round/square, press morph; default S 40×16; outlined = outline-variant",
    },
    ComponentEntry {
        name: "Button group",
        material: "Connected button groups (Expressive)",
        docs: "https://m3.material.io/components/button-groups/specs",
        parity: Parity::Done,
        notes: "2dp gap, 8dp inner corners, full-round outer; selected morphs square; Day/Week/Month demo",
    },
    ComponentEntry {
        name: "Icon button",
        material: "Icon buttons",
        docs: "https://m3.material.io/components/icon-buttons/specs",
        parity: Parity::Done,
        notes: "Expressive XS–XL (32/40/56/96/136), round/square, press morph; default S 40×24",
    },
    ComponentEntry {
        name: "FAB",
        material: "Floating action button",
        docs: "https://m3.material.io/components/floating-action-button/specs",
        parity: Parity::Done,
        notes: "Expressive regular 56 / medium 80 / large 96 / small-extended; 40dp small FAB deprecated",
    },
    ComponentEntry {
        name: "Text field",
        material: "Filled and outlined text fields",
        docs: "https://m3.material.io/components/text-fields/specs",
        parity: Parity::Done,
        notes: "Floating label; outlined notch is even-odd hole fill (outer rounded rect minus inner hole minus legend gap) plus PathBuilder stroke; HTML fieldset; 8dp start; focus outline 2dp; IME caret rect stub wired to NativeActivity update_ime_position",
    },
    ComponentEntry {
        name: "List",
        material: "Lists",
        docs: "https://m3.material.io/components/lists/specs",
        parity: Parity::Done,
        notes: "One / two / three line; 56 / 72 / 88dp",
    },
    ComponentEntry {
        name: "Checkbox",
        material: "Checkbox",
        docs: "https://m3.material.io/components/checkbox/specs",
        parity: Parity::Done,
        notes: "18dp / 2dp corners / 48dp target; checked, unchecked, indeterminate",
    },
    ComponentEntry {
        name: "Radio",
        material: "Radio button",
        docs: "https://m3.material.io/components/radio-button/specs",
        parity: Parity::Done,
        notes: "20dp / 48dp target",
    },
    ComponentEntry {
        name: "Switch",
        material: "Switch",
        docs: "https://m3.material.io/components/switch/specs",
        parity: Parity::Done,
        notes: "52×32 track; 16/24dp thumb",
    },
    ComponentEntry {
        name: "Chip",
        material: "Assist / filter / input / suggestion chips",
        docs: "https://m3.material.io/components/chips/specs",
        parity: Parity::Done,
        notes: "32dp height; selected filter/input use secondary container",
    },
    ComponentEntry {
        name: "Card",
        material: "Elevated / filled / outlined cards",
        docs: "https://m3.material.io/components/cards/specs",
        parity: Parity::Done,
        notes: "12dp corners, 16dp padding",
    },
    ComponentEntry {
        name: "Divider",
        material: "Divider",
        docs: "https://m3.material.io/components/divider/specs",
        parity: Parity::Done,
        notes: "1dp outline-variant; full-bleed and inset",
    },
    ComponentEntry {
        name: "Progress",
        material: "Linear and circular progress indicators",
        docs: "https://m3.material.io/components/progress-indicators/specs",
        parity: Parity::Done,
        notes: "Determinate linear/circular + wavy + loading circular; round-capped PTR/loading stroke (PathBuilder + cap discs); shared clock_ms animation helper",
    },
    ComponentEntry {
        name: "Top app bar",
        material: "Small top app bar",
        docs: "https://m3.material.io/components/top-app-bar/specs",
        parity: Parity::Done,
        notes: "64dp surface bar; no medium/large collapsing",
    },
    ComponentEntry {
        name: "Snackbar",
        material: "Snackbar",
        docs: "https://m3.material.io/components/snackbar/specs",
        parity: Parity::Done,
        notes: "Visual only; no timeout / swipe-to-dismiss runtime",
    },
    ComponentEntry {
        name: "Navigation bar",
        material: "Navigation bar",
        docs: "https://m3.material.io/components/navigation-bar/specs",
        parity: Parity::Done,
        notes: "80dp; active indicator 64×32; 3 destinations in catalog",
    },
    ComponentEntry {
        name: "Navigation rail",
        material: "Navigation rail",
        docs: "https://m3.material.io/components/navigation-rail/specs",
        parity: Parity::Done,
        notes: "Collapsed 80dp; expanded is a 220dp modal column over a 32% scrim (M3 pattern); FAB toggles; destination selection kept; 56×32 indicator; badges",
    },
    ComponentEntry {
        name: "Dialog",
        material: "Basic dialogs",
        docs: "https://m3.material.io/components/dialogs/specs",
        parity: Parity::Done,
        notes: "surface-container-high, 28dp, elev 3, headlineSmallEmphasized/bodyMedium, 32% scrim; Reset settings guidelines account list + text Cancel/Accept",
    },
    ComponentEntry {
        name: "Bottom sheet",
        material: "Bottom sheets",
        docs: "https://m3.material.io/components/bottom-sheets/specs",
        parity: Parity::Done,
        notes: "surface-container-low, extra-large top 28dp, 32×4 handle, elev 1",
    },
    ComponentEntry {
        name: "Menu",
        material: "Menus",
        docs: "https://m3.material.io/components/menus/specs",
        parity: Parity::Done,
        notes: "surface-container, 4dp, elev 2, 48dp items; selected secondary-container",
    },
    ComponentEntry {
        name: "Slider",
        material: "Sliders",
        docs: "https://m3.material.io/components/sliders/specs",
        parity: Parity::Done,
        notes: "Expressive XS default: 16dp track, 4×44 handle token / ~28dp painted, 6dp gap, 4dp stops; volume-row labels; dual-handle range with local-X drag, optional 5% tick-snap while dragging, min-span 5% (documented on the label), keyboard arrows, 5% click-step; inactive = surface-container-highest; S–XL sizes",
    },
    ComponentEntry {
        name: "Tabs",
        material: "Tabs",
        docs: "https://m3.material.io/components/tabs/specs",
        parity: Parity::Done,
        notes: "Primary 48dp + 3dp primary indicator; secondary 2dp full-width",
    },
    ComponentEntry {
        name: "Badge",
        material: "Badges",
        docs: "https://m3.material.io/components/badges/specs",
        parity: Parity::Done,
        notes: "Small 6dp / large 16dp; error/on-error; 999+",
    },
    ComponentEntry {
        name: "Date picker",
        material: "Date pickers",
        docs: "https://m3.material.io/components/date-pickers/specs",
        parity: Parity::Done,
        notes: "Modal calendar; 40dp days; selected/today/in-range/out-of-month; Sunday-first grid; range hero + docked popup (shadow, select dismiss, outside-click dismiss, month nav that re-grids HTML+GPUI); month ▾ chrome; emphasized large date",
    },
    ComponentEntry {
        name: "Search",
        material: "Search bar + view",
        docs: "https://m3.material.io/components/search/specs",
        parity: Parity::Done,
        notes: "56dp full-round docked bar growing-bar morph into full-screen search activity (spatial-fast height/corners); caret editor + filtered suggestions; HTML morph container (no display:none swap)",
    },
    ComponentEntry {
        name: "Time picker",
        material: "Time pickers (dial)",
        docs: "https://m3.material.io/components/time-pickers/specs",
        parity: Parity::Done,
        notes: "12-hour + minute polar dial; analog hand is a shared filled path with spatial-fast angle lerp on face/value change (hour-face hand motion); displaySmallEmphasized header; AM/PM",
    },
    ComponentEntry {
        name: "Carousel",
        material: "Carousel (hero / multi-browse)",
        docs: "https://m3.material.io/components/carousel/specs",
        parity: Parity::Done,
        notes: "Hero stub: 256dp large item + 120dp neighbors, 8dp gap, extra-large 28dp corners; click snap + velocity/decay fling that can skip more than one item",
    },
];

pub fn markdown_table() -> String {
    let mut out = String::from(
        "| Component | Material Design | Parity | Notes | Docs |\n|---|---|---|---|---|\n",
    );
    for e in INVENTORY {
        out.push_str(&format!(
            "| {} | {} | {} | {} | [spec]({}) |\n",
            e.name,
            e.material,
            e.parity.label(),
            e.notes,
            e.docs
        ));
    }
    out
}
