use crate::dioxus_elements::math::display;
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, info};

#[component]
pub fn TestComponent() -> Element {
    rsx! {
        div { class: "lg:col-span-2",
            div {
                class: "rounded-lg border bg-card text-card-foreground shadow-2xs",
                "data-v0-t": "card",
                div { class: "space-y-1.5 p-6 flex flex-row items-center justify-between",
                    h3 { class: "text-2xl font-semibold leading-none tracking-tight",
                        "Queue Status"
                    }
                    div { class: "flex items-center gap-2",
                        button { class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 border-input bg-background hover:bg-accent hover:text-accent-foreground border h-9 rounded-md px-3",
                            svg {
                                class: "lucide lucide-skip-forward w-4 h-4 mr-2",
                                fill: "none",
                                height: "24",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                width: "24",
                                xmlns: "http://www.w3.org/2000/svg",
                                polygon { points: "5 4 15 12 5 20 5 4" }
                                line {
                                    x1: "19",
                                    x2: "19",
                                    y1: "5",
                                    y2: "19",
                                }
                            }
                            "Complete Current Song"
                        }
                    }
                }
                div { class: "p-6 pt-0",
                    div { class: "bg-green-100 dark:bg-green-900 p-4 rounded-lg mb-6",
                        h3 { class: "font-bold text-lg mb-2 flex items-center gap-2",
                            "🎤 Currently Singing"
                        }
                        div { class: "flex items-center justify-between",
                            div {
                                div { class: "text-xl font-semibold", "Queen" }
                                div { class: "text-lg text-muted-foreground", "Bohemian Rhapsody" }
                                div { class: "text-lg font-medium text-green-700 dark:text-green-300",
                                    "👤 Alice"
                                }
                            }
                            div { class: "flex gap-1",
                                button {
                                    class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 border-input bg-background hover:bg-accent hover:text-accent-foreground border h-9 rounded-md px-3 text-xs",
                                    title: "Search on iSing",
                                    svg {
                                        class: "lucide lucide-external-link w-3 h-3 mr-1",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M15 3h6v6" }
                                        path { d: "M10 14 21 3" }
                                        path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                                    }
                                    "iSing"
                                }
                                button {
                                    class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 border-input bg-background hover:bg-accent hover:text-accent-foreground border h-9 rounded-md px-3 text-xs",
                                    title: "Open direct YouTube link",
                                    svg {
                                        class: "lucide lucide-external-link w-3 h-3 mr-1",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M15 3h6v6" }
                                        path { d: "M10 14 21 3" }
                                        path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                                    }
                                    "YouTube"
                                }
                            }
                        }
                    }
                    h3 { class: "font-bold text-lg mb-4 flex items-center gap-2", "🔜 Up Next (3)" }
                    div {
                        class: "relative overflow-hidden h-64 mb-6",
                        dir: "ltr",
                        style: "position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px;",

                        div {
                            class: "h-full w-full rounded-[inherit]",
                            "data-radix-scroll-area-viewport": "",
                            style: "overflow: hidden scroll;",
                            div { style: "min-width: 100%; display: table;",
                                div { class: "space-y-2",
                                    div {
                                        class: "rounded-lg border bg-card text-card-foreground shadow-2xs p-3 transition-all duration-200 cursor-not-allowed opacity-60",
                                        "data-v0-t": "card",
                                        draggable: "false",
                                        div { class: "flex items-center gap-3",
                                            div {
                                                class: "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-hidden focus:ring-2 focus:ring-ring focus:ring-offset-2 text-foreground flex-shrink-0",
                                                "data-v0-t": "badge",
                                                "#1"
                                            }
                                            div { class: "flex-1 min-w-0",
                                                div { class: "font-medium truncate",
                                                    "Neil Diamond"
                                                }
                                                div { class: "text-sm text-m uted-foreground truncate",
                                                    "Sweet Caroline • 👤 Bob"
                                                }
                                            }
                                            div { class: "flex items-center gap-2 flex-shrink-0",
                                                div { class: "flex gap-1",
                                                    button {
                                                        class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 border-input bg-background hover:bg-accent hover:text-accent-foreground border h-9 rounded-md px-3 text-xs",
                                                        title: "Search on iSing",
                                                        svg {
                                                            class: "lucide lucide-external-link w-3 h-3 mr-1",
                                                            fill: "none",
                                                            height: "24",
                                                            stroke: "currentColor",
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            stroke_width: "2",
                                                            view_box: "0 0 24 24",
                                                            width: "24",
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            path { d: "M15 3h6v6" }
                                                            path { d: "M10 14 21 3" }
                                                            path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                                                        }
                                                        "iSing"
                                                    }
                                                    button {
                                                        class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 border-input bg-background hover:bg-accent hover:text-accent-foreground border h-9 rounded-md px-3 text-xs",
                                                        title: "Search on YouTube",
                                                        svg {
                                                            class: "lucide lucide-external-link w-3 h-3 mr-1",
                                                            fill: "none",
                                                            height: "24",
                                                            stroke: "currentColor",
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            stroke_width: "2",
                                                            view_box: "0 0 24 24",
                                                            width: "24",
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            path { d: "M15 3h6v6" }
                                                            path { d: "M10 14 21 3" }
                                                            path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                                                        }
                                                        "YouTube"
                                                    }
                                                }
                                                button {
                                                    class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3",
                                                    disabled: "",
                                                    svg {
                                                        class: "lucide lucide-trash2 w-4 h-4",
                                                        fill: "none",
                                                        height: "24",
                                                        stroke: "currentColor",
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "2",
                                                        view_box: "0 0 24 24",
                                                        width: "24",
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        path { d: "M3 6h18" }
                                                        path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                                        path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                                        line {
                                                            x1: "10",
                                                            x2: "10",
                                                            y1: "11",
                                                            y2: "17",
                                                        }
                                                        line {
                                                            x1: "14",
                                                            x2: "14",
                                                            y1: "11",
                                                            y2: "17",
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        class: "rounded-lg border bg-card text-card-foreground shadow-2xs p-3 transition-all duration-200 cursor-not-allowed opacity-60",
                                        "data-v0-t": "card",
                                        draggable: "false",
                                        div { class: "flex items-center gap-3",
                                            div {
                                                class: "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-hidden focus:ring-2 focus:ring-ring focus:ring-offset-2 text-foreground flex-shrink-0",
                                                "data-v0-t": "badge",
                                                "#2"
                                            }
                                            div { class: "flex-1 min-w-0",
                                                div { class: "font-medium truncate",
                                                    "Journey"
                                                }
                                                div { class: "text-sm text-m uted-foreground truncate",
                                                    "Don't Stop Believin' • 👤 Charlie"
                                                }
                                            }
                                            div { class: "flex items-center gap-2 flex-shrink-0",
                                                div { class: "flex gap-1",
                                                    button {
                                                        class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 border-input bg-background hover:bg-accent hover:text-accent-foreground border h-9 rounded-md px-3 text-xs",
                                                        title: "Open direct iSing link",
                                                        svg {
                                                            class: "lucide lucide-external-link w-3 h-3 mr-1",
                                                            fill: "none",
                                                            height: "24",
                                                            stroke: "currentColor",
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            stroke_width: "2",
                                                            view_box: "0 0 24 24",
                                                            width: "24",
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            path { d: "M15 3h6v6" }
                                                            path { d: "M10 14 21 3" }
                                                            path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                                                        }
                                                        "iSing"
                                                    }
                                                    button {
                                                        class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 border-input bg-background hover:bg-accent hover:text-accent-foreground border h-9 rounded-md px-3 text-xs",
                                                        title: "Search on YouTube",
                                                        svg {
                                                            class: "lucide lucide-external-link w-3 h-3 mr-1",
                                                            fill: "none",
                                                            height: "24",
                                                            stroke: "currentColor",
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            stroke_width: "2",
                                                            view_box: "0 0 24 24",
                                                            width: "24",
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            path { d: "M15 3h6v6" }
                                                            path { d: "M10 14 21 3" }
                                                            path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                                                        }
                                                        "YouTube"
                                                    }
                                                }
                                                button {
                                                    class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3",
                                                    disabled: "",
                                                    svg {
                                                        class: "lucide lucide-trash2 w-4 h-4",
                                                        fill: "none",
                                                        height: "24",
                                                        stroke: "currentColor",
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "2",
                                                        view_box: "0 0 24 24",
                                                        width: "24",
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        path { d: "M3 6h18" }
                                                        path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                                        path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                                        line {
                                                            x1: "10",
                                                            x2: "10",
                                                            y1: "11",
                                                            y2: "17",
                                                        }
                                                        line {
                                                            x1: "14",
                                                            x2: "14",
                                                            y1: "11",
                                                            y2: "17",
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        class: "rounded-lg border bg-card text-card-foreground shadow-2xs p-3 transition-all duration-200 cursor-not-allowed opacity-60",
                                        "data-v0-t": "card",
                                        draggable: "false",
                                        div { class: "flex items-center gap-3",
                                            div {
                                                class: "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-hidden focus:ring-2 focus:ring-ring focus:ring-offset-2 text-foreground flex-shrink-0",
                                                "data-v0-t": "badge",
                                                "#3"
                                            }
                                            div { class: "flex-1 min-w-0",
                                                div { class: "font-medium truncate",
                                                    "Bon Jovi"
                                                }
                                                div { class: "text-sm text-m uted-foreground truncate",
                                                    "Livin' on a Prayer • 👤 Diana"
                                                }
                                            }
                                            div { class: "flex items-center gap-2 flex-shrink-0",
                                                div { class: "flex gap-1",
                                                    button {
                                                        class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 border-input bg-background hover:bg-accent hover:text-accent-foreground border h-9 rounded-md px-3 text-xs",
                                                        title: "Search on iSing",
                                                        svg {
                                                            class: "lucide lucide-external-link w-3 h-3 mr-1",
                                                            fill: "none",
                                                            height: "24",
                                                            stroke: "currentColor",
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            stroke_width: "2",
                                                            view_box: "0 0 24 24",
                                                            width: "24",
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            path { d: "M15 3h6v6" }
                                                            path { d: "M10 14 21 3" }
                                                            path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                                                        }
                                                        "iSing"
                                                    }
                                                    button {
                                                        class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 border-input bg-background hover:bg-accent hover:text-accent-foreground border h-9 rounded-md px-3 text-xs",
                                                        title: "Search on YouTube",
                                                        svg {
                                                            class: "lucide lucide-external-link w-3 h-3 mr-1",
                                                            fill: "none",
                                                            height: "24",
                                                            stroke: "currentColor",
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            stroke_width: "2",
                                                            view_box: "0 0 24 24",
                                                            width: "24",
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            path { d: "M15 3h6v6" }
                                                            path { d: "M10 14 21 3" }
                                                            path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                                                        }
                                                        "YouTube"
                                                    }
                                                }
                                                button {
                                                    class: "ring-offset-background focus-visible:outline-hidden focus-visible:ring-ring inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3",
                                                    disabled: "",
                                                    svg {
                                                        class: "lucide lucide-trash2 w-4 h-4",
                                                        fill: "none",
                                                        height: "24",
                                                        stroke: "currentColor",
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "2",
                                                        view_box: "0 0 24 24",
                                                        width: "24",
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        path { d: "M3 6h18" }
                                                        path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                                        path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                                        line {
                                                            x1: "10",
                                                            x2: "10",
                                                            y1: "11",
                                                            y2: "17",
                                                        }
                                                        line {
                                                            x1: "14",
                                                            x2: "14",
                                                            y1: "11",
                                                            y2: "17",
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        div { class: "container mx-auto p-6 max-w-6xl",
            div { class: "flex items-center justify-between mb-6",
                h1 { class: "text-3xl font-bold text-base-content", "Queue Management" }
                div { class: "flex items-center gap-4",
                    div { class: "badge badge-primary badge-lg", "6 songs in queue" }
                    div { class: "flex gap-2",
                        button { class: "btn btn-success",
                            svg {
                                class: "lucide lucide-play w-4 h-4 mr-1",
                                fill: "none",
                                height: "24",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                width: "24",
                                xmlns: "http://www.w3.org/2000/svg",
                                polygon { points: "6 3 20 12 6 21 6 3" }
                            }
                            "Finish Current Song"
                        }
                        button { class: "btn btn-secondary",
                            svg {
                                class: "lucide lucide-users w-4 h-4 mr-1",
                                fill: "none",
                                height: "24",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                width: "24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                                circle { cx: "9", cy: "7", r: "4" }
                                path { d: "M22 21v-2a4 4 0 0 0-3-3.87" }
                                path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
                            }
                            "Reorder by Fairness"
                        }
                    }
                }
            }
            div { class: "card bg-accent text-accent-content shadow-lg mb-6",
                div { class: "card-body",
                    div { class: "flex items-center gap-4",
                        div { class: "badge badge-success font-bold", "NOW PLAYING" }
                        div { class: "flex-1",
                            h3 { class: "font-bold text-xl", "Bohemian Rhapsody" }
                            p { class: "opacity-90", "Queen" }
                            div { class: "flex items-center gap-2 mt-2",
                                div { class: "badge badge-primary", "Alice" }
                                div { class: "badge badge-secondary", "Bob" }
                            }
                            p { class: "text-sm opacity-80 mt-1 italic", "\"Epic duet performance!\"" }
                        }
                        button { class: "btn btn-success",
                            svg {
                                class: "lucide lucide-play w-4 h-4 mr-1",
                                fill: "none",
                                height: "24",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                width: "24",
                                xmlns: "http://www.w3.org/2000/svg",
                                polygon { points: "6 3 20 12 6 21 6 3" }
                            }
                            "Finish Song"
                        }
                    }
                }
            }
            h2 { class: "text-2xl font-bold text-base-content mb-4", "Up Next" }
            div { class: "space-y-4",
                div { class: "card bg-base-100 shadow-md",
                    div { class: "card-body",
                        div { class: "flex items-center gap-4",
                            div { class: "flex flex-col gap-2",
                                button {
                                    class: "btn btn-ghost btn-sm",
                                    disabled: "",
                                    "↑"
                                }
                                svg {
                                    class: "lucide lucide-grip-vertical w-4 h-4 text-base-content/50",
                                    fill: "none",
                                    height: "24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    width: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    circle { cx: "9", cy: "12", r: "1" }
                                    circle { cx: "9", cy: "5", r: "1" }
                                    circle { cx: "9", cy: "19", r: "1" }
                                    circle { cx: "15", cy: "12", r: "1" }
                                    circle { cx: "15", cy: "5", r: "1" }
                                    circle { cx: "15", cy: "19", r: "1" }
                                }
                                button { class: "btn btn-ghost btn-sm", "↓" }
                            }
                            div { class: "badge badge-outline font-bold", "#2" }
                            div { class: "flex-1",
                                h3 { class: "font-bold text-lg", "Don't Stop Believin'" }
                                p { class: "text-base-content/70", "Journey" }
                                div { class: "flex items-center gap-2 mt-2",
                                    div { class: "badge badge-secondary", "Charlie" }
                                }
                                p { class: "text-sm text-base-content/60 mt-1 italic",
                                    "\"Crowd favorite!\""
                                }
                            }
                            div { class: "flex gap-2",
                                button { class: "btn btn-ghost btn-sm",
                                    svg {
                                        class: "lucide lucide-pen-line w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M12 20h9" }
                                        path { d: "M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" }
                                    }
                                }
                                button { class: "btn btn-ghost btn-sm text-error",
                                    svg {
                                        class: "lucide lucide-trash2 w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M3 6h18" }
                                        path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                        path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                        line {
                                            x1: "10",
                                            x2: "10",
                                            y1: "11",
                                            y2: "17",
                                        }
                                        line {
                                            x1: "14",
                                            x2: "14",
                                            y1: "11",
                                            y2: "17",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "card bg-base-100 shadow-md",
                    div { class: "card-body",
                        div { class: "flex items-center gap-4",
                            div { class: "flex flex-col gap-2",
                                button { class: "btn btn-ghost btn-sm", "↑" }
                                svg {
                                    class: "lucide lucide-grip-vertical w-4 h-4 text-base-content/50",
                                    fill: "none",
                                    height: "24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    width: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    circle { cx: "9", cy: "12", r: "1" }
                                    circle { cx: "9", cy: "5", r: "1" }
                                    circle { cx: "9", cy: "19", r: "1" }
                                    circle { cx: "15", cy: "12", r: "1" }
                                    circle { cx: "15", cy: "5", r: "1" }
                                    circle { cx: "15", cy: "19", r: "1" }
                                }
                                button { class: "btn btn-ghost btn-sm", "↓" }
                            }
                            div { class: "badge badge-outline font-bold", "#3" }
                            div { class: "flex-1",
                                h3 { class: "font-bold text-lg", "Dancing Queen" }
                                p { class: "text-base-content/70", "ABBA" }
                                div { class: "flex items-center gap-2 mt-2",
                                    div { class: "badge badge-secondary", "Diana" }
                                    div { class: "badge badge-accent", "Eve" }
                                }
                            }
                            div { class: "flex gap-2",
                                button { class: "btn btn-ghost btn-sm",
                                    svg {
                                        class: "lucide lucide-pen-line w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M12 20h9" }
                                        path { d: "M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" }
                                    }
                                }
                                button { class: "btn btn-ghost btn-sm text-error",
                                    svg {
                                        class: "lucide lucide-trash2 w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M3 6h18" }
                                        path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                        path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                        line {
                                            x1: "10",
                                            x2: "10",
                                            y1: "11",
                                            y2: "17",
                                        }
                                        line {
                                            x1: "14",
                                            x2: "14",
                                            y1: "11",
                                            y2: "17",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "card bg-base-100 shadow-md",
                    div { class: "card-body",
                        div { class: "flex items-center gap-4",
                            div { class: "flex flex-col gap-2",
                                button { class: "btn btn-ghost btn-sm", "↑" }
                                svg {
                                    class: "lucide lucide-grip-vertical w-4 h-4 text-base-content/50",
                                    fill: "none",
                                    height: "24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    width: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    circle { cx: "9", cy: "12", r: "1" }
                                    circle { cx: "9", cy: "5", r: "1" }
                                    circle { cx: "9", cy: "19", r: "1" }
                                    circle { cx: "15", cy: "12", r: "1" }
                                    circle { cx: "15", cy: "5", r: "1" }
                                    circle { cx: "15", cy: "19", r: "1" }
                                }
                                button { class: "btn btn-ghost btn-sm", "↓" }
                            }
                            div { class: "badge badge-outline font-bold", "#4" }
                            div { class: "flex-1",
                                h3 { class: "font-bold text-lg", "Sweet Caroline" }
                                p { class: "text-base-content/70", "Neil Diamond" }
                                div { class: "flex items-center gap-2 mt-2",
                                    div { class: "badge badge-secondary", "Alice" }
                                }
                                p { class: "text-sm text-base-content/60 mt-1 italic",
                                    "\"Second song - love this one!\""
                                }
                            }
                            div { class: "flex gap-2",
                                button { class: "btn btn-ghost btn-sm",
                                    svg {
                                        class: "lucide lucide-pen-line w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M12 20h9" }
                                        path { d: "M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" }
                                    }
                                }
                                button { class: "btn btn-ghost btn-sm text-error",
                                    svg {
                                        class: "lucide lucide-trash2 w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M3 6h18" }
                                        path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                        path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                        line {
                                            x1: "10",
                                            x2: "10",
                                            y1: "11",
                                            y2: "17",
                                        }
                                        line {
                                            x1: "14",
                                            x2: "14",
                                            y1: "11",
                                            y2: "17",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "card bg-base-100 shadow-md",
                    div { class: "card-body",
                        div { class: "flex items-center gap-4",
                            div { class: "flex flex-col gap-2",
                                button { class: "btn btn-ghost btn-sm", "↑" }
                                svg {
                                    class: "lucide lucide-grip-vertical w-4 h-4 text-base-content/50",
                                    fill: "none",
                                    height: "24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    width: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    circle { cx: "9", cy: "12", r: "1" }
                                    circle { cx: "9", cy: "5", r: "1" }
                                    circle { cx: "9", cy: "19", r: "1" }
                                    circle { cx: "15", cy: "12", r: "1" }
                                    circle { cx: "15", cy: "5", r: "1" }
                                    circle { cx: "15", cy: "19", r: "1" }
                                }
                                button { class: "btn btn-ghost btn-sm", "↓" }
                            }
                            div { class: "badge badge-outline font-bold", "#5" }
                            div { class: "flex-1",
                                h3 { class: "font-bold text-lg", "Livin' on a Prayer" }
                                p { class: "text-base-content/70", "Bon Jovi" }
                                div { class: "flex items-center gap-2 mt-2",
                                    div { class: "badge badge-secondary", "Frank" }
                                }
                            }
                            div { class: "flex gap-2",
                                button { class: "btn btn-ghost btn-sm",
                                    svg {
                                        class: "lucide lucide-pen-line w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M12 20h9" }
                                        path { d: "M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" }
                                    }
                                }
                                button { class: "btn btn-ghost btn-sm text-error",
                                    svg {
                                        class: "lucide lucide-trash2 w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M3 6h18" }
                                        path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                        path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                        line {
                                            x1: "10",
                                            x2: "10",
                                            y1: "11",
                                            y2: "17",
                                        }
                                        line {
                                            x1: "14",
                                            x2: "14",
                                            y1: "11",
                                            y2: "17",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "card bg-base-100 shadow-md",
                    div { class: "card-body",
                        div { class: "flex items-center gap-4",
                            div { class: "flex flex-col gap-2",
                                button { class: "btn btn-ghost btn-sm", "↑" }
                                svg {
                                    class: "lucide lucide-grip-vertical w-4 h-4 text-base-content/50",
                                    fill: "none",
                                    height: "24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    width: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    circle { cx: "9", cy: "12", r: "1" }
                                    circle { cx: "9", cy: "5", r: "1" }
                                    circle { cx: "9", cy: "19", r: "1" }
                                    circle { cx: "15", cy: "12", r: "1" }
                                    circle { cx: "15", cy: "5", r: "1" }
                                    circle { cx: "15", cy: "19", r: "1" }
                                }
                                button {
                                    class: "btn btn-ghost btn-sm",
                                    disabled: "",
                                    "↓"
                                }
                            }
                            div { class: "badge badge-outline font-bold", "#6" }
                            div { class: "flex-1",
                                h3 { class: "font-bold text-lg", "Wonderwall" }
                                p { class: "text-base-content/70", "Oasis" }
                                div { class: "flex items-center gap-2 mt-2",
                                    div { class: "badge badge-secondary", "Charlie" }
                                    div { class: "badge badge-accent", "Grace" }
                                }
                                p { class: "text-sm text-base-content/60 mt-1 italic",
                                    "\"Acoustic version please\""
                                }
                            }
                            div { class: "flex gap-2",
                                button { class: "btn btn-ghost btn-sm",
                                    svg {
                                        class: "lucide lucide-pen-line w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M12 20h9" }
                                        path { d: "M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" }
                                    }
                                }
                                button { class: "btn btn-ghost btn-sm text-error",
                                    svg {
                                        class: "lucide lucide-trash2 w-4 h-4",
                                        fill: "none",
                                        height: "24",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M3 6h18" }
                                        path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                        path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                        line {
                                            x1: "10",
                                            x2: "10",
                                            y1: "11",
                                            y2: "17",
                                        }
                                        line {
                                            x1: "14",
                                            x2: "14",
                                            y1: "11",
                                            y2: "17",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
