//! DO NOT EDIT MANUALLY - this file is autogenerated from the default Tailwind CSS config.
//! To update, run `pnpm gen:tailwind-preset` from `packages/@biomejs/tailwindcss-config-analyzer`.

use super::sort_config::{build_variant_weight, UtilityLayer, Variant};

const COMPONENTS_LAYER_CLASSES: [&str; 1] = ["container$"];

const UTILITIES_LAYER_CLASSES: [&str; 578] = [
    "sr-only$",
    "not-sr-only$",
    "pointer-events-none$",
    "pointer-events-auto$",
    "visible$",
    "invisible$",
    "collapse$",
    "static$",
    "fixed$",
    "absolute$",
    "relative$",
    "sticky$",
    "inset-",
    "inset-x-",
    "inset-y-",
    "start-",
    "end-",
    "top-",
    "right-",
    "bottom-",
    "left-",
    "isolate$",
    "isolation-auto$",
    "z-",
    "order-",
    "col-",
    "col-start-",
    "col-end-",
    "row-",
    "row-start-",
    "row-end-",
    "float-start$",
    "float-end$",
    "float-right$",
    "float-left$",
    "float-none$",
    "clear-start$",
    "clear-end$",
    "clear-left$",
    "clear-right$",
    "clear-both$",
    "clear-none$",
    "m-",
    "mx-",
    "my-",
    "ms-",
    "me-",
    "mt-",
    "mr-",
    "mb-",
    "ml-",
    "box-border$",
    "box-content$",
    "line-clamp-",
    "line-clamp-none$",
    "block$",
    "inline-block$",
    "inline$",
    "flex$",
    "inline-flex$",
    "table$",
    "inline-table$",
    "table-caption$",
    "table-cell$",
    "table-column$",
    "table-column-group$",
    "table-footer-group$",
    "table-header-group$",
    "table-row-group$",
    "table-row$",
    "flow-root$",
    "grid$",
    "inline-grid$",
    "contents$",
    "list-item$",
    "hidden$",
    "aspect-",
    "size-",
    "h-",
    "max-h-",
    "min-h-",
    "w-",
    "min-w-",
    "max-w-",
    "flex-",
    "flex-shrink$",
    "flex-shrink-",
    "shrink$",
    "shrink-",
    "flex-grow$",
    "flex-grow-",
    "grow$",
    "grow-",
    "basis-",
    "table-auto$",
    "table-fixed$",
    "caption-top$",
    "caption-bottom$",
    "border-collapse$",
    "border-separate$",
    "border-spacing-",
    "border-spacing-x-",
    "border-spacing-y-",
    "origin-",
    "translate-x-",
    "translate-y-",
    "rotate-",
    "skew-x-",
    "skew-y-",
    "scale-",
    "scale-x-",
    "scale-y-",
    "transform$",
    "transform-cpu$",
    "transform-gpu$",
    "transform-none$",
    "animate-",
    "cursor-",
    "touch-auto$",
    "touch-none$",
    "touch-pan-x$",
    "touch-pan-left$",
    "touch-pan-right$",
    "touch-pan-y$",
    "touch-pan-up$",
    "touch-pan-down$",
    "touch-pinch-zoom$",
    "touch-manipulation$",
    "select-none$",
    "select-text$",
    "select-all$",
    "select-auto$",
    "resize-none$",
    "resize-y$",
    "resize-x$",
    "resize$",
    "snap-none$",
    "snap-x$",
    "snap-y$",
    "snap-both$",
    "snap-mandatory$",
    "snap-proximity$",
    "snap-start$",
    "snap-end$",
    "snap-center$",
    "snap-align-none$",
    "snap-normal$",
    "snap-always$",
    "scroll-m-",
    "scroll-mx-",
    "scroll-my-",
    "scroll-ms-",
    "scroll-me-",
    "scroll-mt-",
    "scroll-mr-",
    "scroll-mb-",
    "scroll-ml-",
    "scroll-p-",
    "scroll-px-",
    "scroll-py-",
    "scroll-ps-",
    "scroll-pe-",
    "scroll-pt-",
    "scroll-pr-",
    "scroll-pb-",
    "scroll-pl-",
    "list-inside$",
    "list-outside$",
    "list-",
    "list-image-",
    "appearance-none$",
    "appearance-auto$",
    "columns-",
    "break-before-auto$",
    "break-before-avoid$",
    "break-before-all$",
    "break-before-avoid-page$",
    "break-before-page$",
    "break-before-left$",
    "break-before-right$",
    "break-before-column$",
    "break-inside-auto$",
    "break-inside-avoid$",
    "break-inside-avoid-page$",
    "break-inside-avoid-column$",
    "break-after-auto$",
    "break-after-avoid$",
    "break-after-all$",
    "break-after-avoid-page$",
    "break-after-page$",
    "break-after-left$",
    "break-after-right$",
    "break-after-column$",
    "auto-cols-",
    "grid-flow-row$",
    "grid-flow-col$",
    "grid-flow-dense$",
    "grid-flow-row-dense$",
    "grid-flow-col-dense$",
    "auto-rows-",
    "grid-cols-",
    "grid-rows-",
    "flex-row$",
    "flex-row-reverse$",
    "flex-col$",
    "flex-col-reverse$",
    "flex-wrap$",
    "flex-wrap-reverse$",
    "flex-nowrap$",
    "place-content-center$",
    "place-content-start$",
    "place-content-end$",
    "place-content-between$",
    "place-content-around$",
    "place-content-evenly$",
    "place-content-baseline$",
    "place-content-stretch$",
    "place-items-start$",
    "place-items-end$",
    "place-items-center$",
    "place-items-baseline$",
    "place-items-stretch$",
    "content-normal$",
    "content-center$",
    "content-start$",
    "content-end$",
    "content-between$",
    "content-around$",
    "content-evenly$",
    "content-baseline$",
    "content-stretch$",
    "items-start$",
    "items-end$",
    "items-center$",
    "items-baseline$",
    "items-stretch$",
    "justify-normal$",
    "justify-start$",
    "justify-end$",
    "justify-center$",
    "justify-between$",
    "justify-around$",
    "justify-evenly$",
    "justify-stretch$",
    "justify-items-start$",
    "justify-items-end$",
    "justify-items-center$",
    "justify-items-stretch$",
    "gap-",
    "gap-x-",
    "gap-y-",
    "space-x-",
    "space-y-",
    "space-y-reverse$",
    "space-x-reverse$",
    "divide-x$",
    "divide-x-",
    "divide-y$",
    "divide-y-",
    "divide-y-reverse$",
    "divide-x-reverse$",
    "divide-solid$",
    "divide-dashed$",
    "divide-dotted$",
    "divide-double$",
    "divide-none$",
    "divide-",
    "divide-opacity-",
    "place-self-auto$",
    "place-self-start$",
    "place-self-end$",
    "place-self-center$",
    "place-self-stretch$",
    "self-auto$",
    "self-start$",
    "self-end$",
    "self-center$",
    "self-stretch$",
    "self-baseline$",
    "justify-self-auto$",
    "justify-self-start$",
    "justify-self-end$",
    "justify-self-center$",
    "justify-self-stretch$",
    "overflow-auto$",
    "overflow-hidden$",
    "overflow-clip$",
    "overflow-visible$",
    "overflow-scroll$",
    "overflow-x-auto$",
    "overflow-y-auto$",
    "overflow-x-hidden$",
    "overflow-y-hidden$",
    "overflow-x-clip$",
    "overflow-y-clip$",
    "overflow-x-visible$",
    "overflow-y-visible$",
    "overflow-x-scroll$",
    "overflow-y-scroll$",
    "overscroll-auto$",
    "overscroll-contain$",
    "overscroll-none$",
    "overscroll-y-auto$",
    "overscroll-y-contain$",
    "overscroll-y-none$",
    "overscroll-x-auto$",
    "overscroll-x-contain$",
    "overscroll-x-none$",
    "scroll-auto$",
    "scroll-smooth$",
    "truncate$",
    "overflow-ellipsis$",
    "text-ellipsis$",
    "text-clip$",
    "hyphens-none$",
    "hyphens-manual$",
    "hyphens-auto$",
    "whitespace-normal$",
    "whitespace-nowrap$",
    "whitespace-pre$",
    "whitespace-pre-line$",
    "whitespace-pre-wrap$",
    "whitespace-break-spaces$",
    "text-wrap$",
    "text-nowrap$",
    "text-balance$",
    "text-pretty$",
    "break-normal$",
    "break-words$",
    "break-all$",
    "break-keep$",
    "rounded$",
    "rounded-",
    "rounded-s$",
    "rounded-s-",
    "rounded-e$",
    "rounded-e-",
    "rounded-t$",
    "rounded-t-",
    "rounded-r$",
    "rounded-r-",
    "rounded-b$",
    "rounded-b-",
    "rounded-l$",
    "rounded-l-",
    "rounded-ss$",
    "rounded-ss-",
    "rounded-se$",
    "rounded-se-",
    "rounded-ee$",
    "rounded-ee-",
    "rounded-es$",
    "rounded-es-",
    "rounded-tl$",
    "rounded-tl-",
    "rounded-tr$",
    "rounded-tr-",
    "rounded-br$",
    "rounded-br-",
    "rounded-bl$",
    "rounded-bl-",
    "border$",
    "border-",
    "border-x$",
    "border-x-",
    "border-y$",
    "border-y-",
    "border-s$",
    "border-s-",
    "border-e$",
    "border-e-",
    "border-t$",
    "border-t-",
    "border-r$",
    "border-r-",
    "border-b$",
    "border-b-",
    "border-l$",
    "border-l-",
    "border-solid$",
    "border-dashed$",
    "border-dotted$",
    "border-double$",
    "border-hidden$",
    "border-none$",
    "border-opacity-",
    "bg-",
    "bg-opacity-",
    "from-",
    "via-",
    "to-",
    "decoration-slice$",
    "decoration-clone$",
    "box-decoration-slice$",
    "box-decoration-clone$",
    "bg-fixed$",
    "bg-local$",
    "bg-scroll$",
    "bg-clip-border$",
    "bg-clip-padding$",
    "bg-clip-content$",
    "bg-clip-text$",
    "bg-repeat$",
    "bg-no-repeat$",
    "bg-repeat-x$",
    "bg-repeat-y$",
    "bg-repeat-round$",
    "bg-repeat-space$",
    "bg-origin-border$",
    "bg-origin-padding$",
    "bg-origin-content$",
    "fill-",
    "stroke-",
    "object-contain$",
    "object-cover$",
    "object-fill$",
    "object-none$",
    "object-scale-down$",
    "object-",
    "p-",
    "px-",
    "py-",
    "ps-",
    "pe-",
    "pt-",
    "pr-",
    "pb-",
    "pl-",
    "text-left$",
    "text-center$",
    "text-right$",
    "text-justify$",
    "text-start$",
    "text-end$",
    "indent-",
    "align-baseline$",
    "align-top$",
    "align-middle$",
    "align-bottom$",
    "align-text-top$",
    "align-text-bottom$",
    "align-sub$",
    "align-super$",
    "align-",
    "font-",
    "text-",
    "uppercase$",
    "lowercase$",
    "capitalize$",
    "normal-case$",
    "italic$",
    "not-italic$",
    "normal-nums$",
    "ordinal$",
    "slashed-zero$",
    "lining-nums$",
    "oldstyle-nums$",
    "proportional-nums$",
    "tabular-nums$",
    "diagonal-fractions$",
    "stacked-fractions$",
    "leading-",
    "tracking-",
    "text-opacity-",
    "underline$",
    "overline$",
    "line-through$",
    "no-underline$",
    "decoration-",
    "decoration-solid$",
    "decoration-double$",
    "decoration-dotted$",
    "decoration-dashed$",
    "decoration-wavy$",
    "underline-offset-",
    "antialiased$",
    "subpixel-antialiased$",
    "placeholder-",
    "placeholder-opacity-",
    "caret-",
    "accent-",
    "opacity-",
    "bg-blend-normal$",
    "bg-blend-multiply$",
    "bg-blend-screen$",
    "bg-blend-overlay$",
    "bg-blend-darken$",
    "bg-blend-lighten$",
    "bg-blend-color-dodge$",
    "bg-blend-color-burn$",
    "bg-blend-hard-light$",
    "bg-blend-soft-light$",
    "bg-blend-difference$",
    "bg-blend-exclusion$",
    "bg-blend-hue$",
    "bg-blend-saturation$",
    "bg-blend-color$",
    "bg-blend-luminosity$",
    "mix-blend-normal$",
    "mix-blend-multiply$",
    "mix-blend-screen$",
    "mix-blend-overlay$",
    "mix-blend-darken$",
    "mix-blend-lighten$",
    "mix-blend-color-dodge$",
    "mix-blend-color-burn$",
    "mix-blend-hard-light$",
    "mix-blend-soft-light$",
    "mix-blend-difference$",
    "mix-blend-exclusion$",
    "mix-blend-hue$",
    "mix-blend-saturation$",
    "mix-blend-color$",
    "mix-blend-luminosity$",
    "mix-blend-plus-darker$",
    "mix-blend-plus-lighter$",
    "shadow$",
    "shadow-",
    "outline-none$",
    "outline$",
    "outline-dashed$",
    "outline-dotted$",
    "outline-double$",
    "outline-",
    "outline-offset-",
    "ring$",
    "ring-",
    "ring-inset$",
    "ring-opacity-",
    "ring-offset-",
    "blur$",
    "blur-",
    "brightness-",
    "contrast-",
    "drop-shadow$",
    "drop-shadow-",
    "grayscale$",
    "grayscale-",
    "hue-rotate-",
    "invert$",
    "invert-",
    "saturate-",
    "sepia$",
    "sepia-",
    "filter$",
    "filter-none$",
    "backdrop-blur$",
    "backdrop-blur-",
    "backdrop-brightness-",
    "backdrop-contrast-",
    "backdrop-grayscale$",
    "backdrop-grayscale-",
    "backdrop-hue-rotate-",
    "backdrop-invert$",
    "backdrop-invert-",
    "backdrop-opacity-",
    "backdrop-saturate-",
    "backdrop-sepia$",
    "backdrop-sepia-",
    "backdrop-filter$",
    "backdrop-filter-none$",
    "transition$",
    "transition-",
    "delay-",
    "duration-",
    "ease-",
    "will-change-",
    "contain-none$",
    "contain-content$",
    "contain-strict$",
    "contain-size$",
    "contain-inline-size$",
    "contain-layout$",
    "contain-paint$",
    "contain-style$",
    "content-",
    "forced-color-adjust-auto$",
    "forced-color-adjust-none$",
];

pub fn get_variant_classes() -> Vec<Variant> {
    vec![
        Variant {
            name: "*",
            weight: build_variant_weight::<0>(),
        },
        Variant {
            name: "first-letter",
            weight: build_variant_weight::<1>(),
        },
        Variant {
            name: "first-line",
            weight: build_variant_weight::<2>(),
        },
        Variant {
            name: "marker",
            weight: build_variant_weight::<3>(),
        },
        Variant {
            name: "selection",
            weight: build_variant_weight::<5>(),
        },
        Variant {
            name: "file",
            weight: build_variant_weight::<7>(),
        },
        Variant {
            name: "placeholder",
            weight: build_variant_weight::<8>(),
        },
        Variant {
            name: "backdrop",
            weight: build_variant_weight::<9>(),
        },
        Variant {
            name: "before",
            weight: build_variant_weight::<10>(),
        },
        Variant {
            name: "after",
            weight: build_variant_weight::<11>(),
        },
        Variant {
            name: "first",
            weight: build_variant_weight::<12>(),
        },
        Variant {
            name: "last",
            weight: build_variant_weight::<13>(),
        },
        Variant {
            name: "only",
            weight: build_variant_weight::<14>(),
        },
        Variant {
            name: "odd",
            weight: build_variant_weight::<15>(),
        },
        Variant {
            name: "even",
            weight: build_variant_weight::<16>(),
        },
        Variant {
            name: "first-of-type",
            weight: build_variant_weight::<17>(),
        },
        Variant {
            name: "last-of-type",
            weight: build_variant_weight::<18>(),
        },
        Variant {
            name: "only-of-type",
            weight: build_variant_weight::<19>(),
        },
        Variant {
            name: "visited",
            weight: build_variant_weight::<20>(),
        },
        Variant {
            name: "target",
            weight: build_variant_weight::<21>(),
        },
        Variant {
            name: "open",
            weight: build_variant_weight::<22>(),
        },
        Variant {
            name: "default",
            weight: build_variant_weight::<23>(),
        },
        Variant {
            name: "checked",
            weight: build_variant_weight::<24>(),
        },
        Variant {
            name: "indeterminate",
            weight: build_variant_weight::<25>(),
        },
        Variant {
            name: "placeholder-shown",
            weight: build_variant_weight::<26>(),
        },
        Variant {
            name: "autofill",
            weight: build_variant_weight::<27>(),
        },
        Variant {
            name: "optional",
            weight: build_variant_weight::<28>(),
        },
        Variant {
            name: "required",
            weight: build_variant_weight::<29>(),
        },
        Variant {
            name: "valid",
            weight: build_variant_weight::<30>(),
        },
        Variant {
            name: "invalid",
            weight: build_variant_weight::<31>(),
        },
        Variant {
            name: "in-range",
            weight: build_variant_weight::<32>(),
        },
        Variant {
            name: "out-of-range",
            weight: build_variant_weight::<33>(),
        },
        Variant {
            name: "read-only",
            weight: build_variant_weight::<34>(),
        },
        Variant {
            name: "empty",
            weight: build_variant_weight::<35>(),
        },
        Variant {
            name: "focus-within",
            weight: build_variant_weight::<36>(),
        },
        Variant {
            name: "hover",
            weight: build_variant_weight::<37>(),
        },
        Variant {
            name: "focus",
            weight: build_variant_weight::<38>(),
        },
        Variant {
            name: "focus-visible",
            weight: build_variant_weight::<39>(),
        },
        Variant {
            name: "active",
            weight: build_variant_weight::<40>(),
        },
        Variant {
            name: "enabled",
            weight: build_variant_weight::<41>(),
        },
        Variant {
            name: "disabled",
            weight: build_variant_weight::<42>(),
        },
        Variant {
            name: "group-first",
            weight: build_variant_weight::<43>(),
        },
        Variant {
            name: "group-last",
            weight: build_variant_weight::<44>(),
        },
        Variant {
            name: "group-only",
            weight: build_variant_weight::<45>(),
        },
        Variant {
            name: "group-odd",
            weight: build_variant_weight::<46>(),
        },
        Variant {
            name: "group-even",
            weight: build_variant_weight::<47>(),
        },
        Variant {
            name: "group-first-of-type",
            weight: build_variant_weight::<48>(),
        },
        Variant {
            name: "group-last-of-type",
            weight: build_variant_weight::<49>(),
        },
        Variant {
            name: "group-only-of-type",
            weight: build_variant_weight::<50>(),
        },
        Variant {
            name: "group-visited",
            weight: build_variant_weight::<51>(),
        },
        Variant {
            name: "group-target",
            weight: build_variant_weight::<52>(),
        },
        Variant {
            name: "group-open",
            weight: build_variant_weight::<53>(),
        },
        Variant {
            name: "group-default",
            weight: build_variant_weight::<54>(),
        },
        Variant {
            name: "group-checked",
            weight: build_variant_weight::<55>(),
        },
        Variant {
            name: "group-indeterminate",
            weight: build_variant_weight::<56>(),
        },
        Variant {
            name: "group-placeholder-shown",
            weight: build_variant_weight::<57>(),
        },
        Variant {
            name: "group-autofill",
            weight: build_variant_weight::<58>(),
        },
        Variant {
            name: "group-optional",
            weight: build_variant_weight::<59>(),
        },
        Variant {
            name: "group-required",
            weight: build_variant_weight::<60>(),
        },
        Variant {
            name: "group-valid",
            weight: build_variant_weight::<61>(),
        },
        Variant {
            name: "group-invalid",
            weight: build_variant_weight::<62>(),
        },
        Variant {
            name: "group-in-range",
            weight: build_variant_weight::<63>(),
        },
        Variant {
            name: "group-out-of-range",
            weight: build_variant_weight::<64>(),
        },
        Variant {
            name: "group-read-only",
            weight: build_variant_weight::<65>(),
        },
        Variant {
            name: "group-empty",
            weight: build_variant_weight::<66>(),
        },
        Variant {
            name: "group-focus-within",
            weight: build_variant_weight::<67>(),
        },
        Variant {
            name: "group-hover",
            weight: build_variant_weight::<68>(),
        },
        Variant {
            name: "group-focus",
            weight: build_variant_weight::<69>(),
        },
        Variant {
            name: "group-focus-visible",
            weight: build_variant_weight::<70>(),
        },
        Variant {
            name: "group-active",
            weight: build_variant_weight::<71>(),
        },
        Variant {
            name: "group-enabled",
            weight: build_variant_weight::<72>(),
        },
        Variant {
            name: "group-disabled",
            weight: build_variant_weight::<73>(),
        },
        Variant {
            name: "group",
            weight: build_variant_weight::<74>(),
        },
        Variant {
            name: "peer-first",
            weight: build_variant_weight::<75>(),
        },
        Variant {
            name: "peer-last",
            weight: build_variant_weight::<76>(),
        },
        Variant {
            name: "peer-only",
            weight: build_variant_weight::<77>(),
        },
        Variant {
            name: "peer-odd",
            weight: build_variant_weight::<78>(),
        },
        Variant {
            name: "peer-even",
            weight: build_variant_weight::<79>(),
        },
        Variant {
            name: "peer-first-of-type",
            weight: build_variant_weight::<80>(),
        },
        Variant {
            name: "peer-last-of-type",
            weight: build_variant_weight::<81>(),
        },
        Variant {
            name: "peer-only-of-type",
            weight: build_variant_weight::<82>(),
        },
        Variant {
            name: "peer-visited",
            weight: build_variant_weight::<83>(),
        },
        Variant {
            name: "peer-target",
            weight: build_variant_weight::<84>(),
        },
        Variant {
            name: "peer-open",
            weight: build_variant_weight::<85>(),
        },
        Variant {
            name: "peer-default",
            weight: build_variant_weight::<86>(),
        },
        Variant {
            name: "peer-checked",
            weight: build_variant_weight::<87>(),
        },
        Variant {
            name: "peer-indeterminate",
            weight: build_variant_weight::<88>(),
        },
        Variant {
            name: "peer-placeholder-shown",
            weight: build_variant_weight::<89>(),
        },
        Variant {
            name: "peer-autofill",
            weight: build_variant_weight::<90>(),
        },
        Variant {
            name: "peer-optional",
            weight: build_variant_weight::<91>(),
        },
        Variant {
            name: "peer-required",
            weight: build_variant_weight::<92>(),
        },
        Variant {
            name: "peer-valid",
            weight: build_variant_weight::<93>(),
        },
        Variant {
            name: "peer-invalid",
            weight: build_variant_weight::<94>(),
        },
        Variant {
            name: "peer-in-range",
            weight: build_variant_weight::<95>(),
        },
        Variant {
            name: "peer-out-of-range",
            weight: build_variant_weight::<96>(),
        },
        Variant {
            name: "peer-read-only",
            weight: build_variant_weight::<97>(),
        },
        Variant {
            name: "peer-empty",
            weight: build_variant_weight::<98>(),
        },
        Variant {
            name: "peer-focus-within",
            weight: build_variant_weight::<99>(),
        },
        Variant {
            name: "peer-hover",
            weight: build_variant_weight::<100>(),
        },
        Variant {
            name: "peer-focus",
            weight: build_variant_weight::<101>(),
        },
        Variant {
            name: "peer-focus-visible",
            weight: build_variant_weight::<102>(),
        },
        Variant {
            name: "peer-active",
            weight: build_variant_weight::<103>(),
        },
        Variant {
            name: "peer-enabled",
            weight: build_variant_weight::<104>(),
        },
        Variant {
            name: "peer-disabled",
            weight: build_variant_weight::<105>(),
        },
        Variant {
            name: "peer",
            weight: build_variant_weight::<106>(),
        },
        Variant {
            name: "has",
            weight: build_variant_weight::<107>(),
        },
        Variant {
            name: "group-has",
            weight: build_variant_weight::<108>(),
        },
        Variant {
            name: "peer-has",
            weight: build_variant_weight::<109>(),
        },
        Variant {
            name: "aria-busy",
            weight: build_variant_weight::<110>(),
        },
        Variant {
            name: "aria-checked",
            weight: build_variant_weight::<111>(),
        },
        Variant {
            name: "aria-disabled",
            weight: build_variant_weight::<112>(),
        },
        Variant {
            name: "aria-expanded",
            weight: build_variant_weight::<113>(),
        },
        Variant {
            name: "aria-hidden",
            weight: build_variant_weight::<114>(),
        },
        Variant {
            name: "aria-pressed",
            weight: build_variant_weight::<115>(),
        },
        Variant {
            name: "aria-readonly",
            weight: build_variant_weight::<116>(),
        },
        Variant {
            name: "aria-required",
            weight: build_variant_weight::<117>(),
        },
        Variant {
            name: "aria-selected",
            weight: build_variant_weight::<118>(),
        },
        Variant {
            name: "aria",
            weight: build_variant_weight::<119>(),
        },
        Variant {
            name: "group-aria-busy",
            weight: build_variant_weight::<120>(),
        },
        Variant {
            name: "group-aria-checked",
            weight: build_variant_weight::<121>(),
        },
        Variant {
            name: "group-aria-disabled",
            weight: build_variant_weight::<122>(),
        },
        Variant {
            name: "group-aria-expanded",
            weight: build_variant_weight::<123>(),
        },
        Variant {
            name: "group-aria-hidden",
            weight: build_variant_weight::<124>(),
        },
        Variant {
            name: "group-aria-pressed",
            weight: build_variant_weight::<125>(),
        },
        Variant {
            name: "group-aria-readonly",
            weight: build_variant_weight::<126>(),
        },
        Variant {
            name: "group-aria-required",
            weight: build_variant_weight::<127>(),
        },
        Variant {
            name: "group-aria-selected",
            weight: build_variant_weight::<128>(),
        },
        Variant {
            name: "group-aria",
            weight: build_variant_weight::<129>(),
        },
        Variant {
            name: "peer-aria-busy",
            weight: build_variant_weight::<130>(),
        },
        Variant {
            name: "peer-aria-checked",
            weight: build_variant_weight::<131>(),
        },
        Variant {
            name: "peer-aria-disabled",
            weight: build_variant_weight::<132>(),
        },
        Variant {
            name: "peer-aria-expanded",
            weight: build_variant_weight::<133>(),
        },
        Variant {
            name: "peer-aria-hidden",
            weight: build_variant_weight::<134>(),
        },
        Variant {
            name: "peer-aria-pressed",
            weight: build_variant_weight::<135>(),
        },
        Variant {
            name: "peer-aria-readonly",
            weight: build_variant_weight::<136>(),
        },
        Variant {
            name: "peer-aria-required",
            weight: build_variant_weight::<137>(),
        },
        Variant {
            name: "peer-aria-selected",
            weight: build_variant_weight::<138>(),
        },
        Variant {
            name: "peer-aria",
            weight: build_variant_weight::<139>(),
        },
        Variant {
            name: "data",
            weight: build_variant_weight::<140>(),
        },
        Variant {
            name: "group-data",
            weight: build_variant_weight::<141>(),
        },
        Variant {
            name: "peer-data",
            weight: build_variant_weight::<142>(),
        },
        Variant {
            name: "supports",
            weight: build_variant_weight::<143>(),
        },
        Variant {
            name: "motion-safe",
            weight: build_variant_weight::<144>(),
        },
        Variant {
            name: "motion-reduce",
            weight: build_variant_weight::<145>(),
        },
        Variant {
            name: "contrast-more",
            weight: build_variant_weight::<146>(),
        },
        Variant {
            name: "contrast-less",
            weight: build_variant_weight::<147>(),
        },
        Variant {
            name: "max-sm",
            weight: build_variant_weight::<148>(),
        },
        Variant {
            name: "max-md",
            weight: build_variant_weight::<149>(),
        },
        Variant {
            name: "max-lg",
            weight: build_variant_weight::<150>(),
        },
        Variant {
            name: "max-xl",
            weight: build_variant_weight::<151>(),
        },
        Variant {
            name: "max-2xl",
            weight: build_variant_weight::<152>(),
        },
        Variant {
            name: "max",
            weight: build_variant_weight::<153>(),
        },
        Variant {
            name: "sm",
            weight: build_variant_weight::<154>(),
        },
        Variant {
            name: "md",
            weight: build_variant_weight::<155>(),
        },
        Variant {
            name: "lg",
            weight: build_variant_weight::<156>(),
        },
        Variant {
            name: "xl",
            weight: build_variant_weight::<157>(),
        },
        Variant {
            name: "2xl",
            weight: build_variant_weight::<158>(),
        },
        Variant {
            name: "min",
            weight: build_variant_weight::<159>(),
        },
        Variant {
            name: "portrait",
            weight: build_variant_weight::<160>(),
        },
        Variant {
            name: "landscape",
            weight: build_variant_weight::<161>(),
        },
        Variant {
            name: "ltr",
            weight: build_variant_weight::<162>(),
        },
        Variant {
            name: "rtl",
            weight: build_variant_weight::<163>(),
        },
        Variant {
            name: "dark",
            weight: build_variant_weight::<164>(),
        },
        Variant {
            name: "forced-colors",
            weight: build_variant_weight::<165>(),
        },
        Variant {
            name: "print",
            weight: build_variant_weight::<166>(),
        },
    ]
}

pub const TAILWIND_LAYERS: [UtilityLayer; 2] = [
    UtilityLayer {
        name: "components",
        classes: COMPONENTS_LAYER_CLASSES.as_slice(),
    },
    UtilityLayer {
        name: "utilities",
        classes: UTILITIES_LAYER_CLASSES.as_slice(),
    },
];
