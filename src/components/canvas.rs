use dioxus::prelude::*;

#[component]
pub fn Canvas() -> Element {
    rsx! {
        div {
          class: "flex flex-col border-2  items-center justify-center w-3/4 rounded-lg",
          svg {
            width: "100%",
            height: "100%",
            view_box: "0 15 100 100",
            circle {
              cx: "50",
              cy: "30",
              r: "5",
              fill: "black",
            }
            line {
              x1: "50",
              y1: "35",
              x2: "30",
              y2: "60",
              stroke: "black",
              stroke_width: "0.5",
              marker_end: "url(#arrowhead)",
            }
            defs {
              marker {
              id: "arrowhead",
              marker_width: "10",
              marker_height: "7",
              ref_x: "0",
              ref_y: "1.5",
              orient: "auto",
                path {
                d: "M0,0 L0,3 L3,1.5 z",
                fill: "black",
                }
              }
            }
            circle {
              cx: "28",
              cy: "66",
              r: "5",
              fill: "black",
            }
            line {
              x1: "50",
              y1: "35",
              x2: "70",
              y2: "60",
              stroke: "black",
              stroke_width: "0.5",
              marker_end: "url(#arrowhead)",
            }
            circle {
              cx: "72",
              cy: "66",
              r: "5",
              fill: "black",
            }
          line {
            x1: "72",
            y1: "66",
            x2: "90",
            y2: "90",
            stroke: "black",
            stroke_width: "0.5",
            marker_end: "url(#arrowhead)",
          }
          circle {
            cx: "92",
            cy: "96",
            r: "5",
            fill: "red",
          }
          }
        }
    }
}
