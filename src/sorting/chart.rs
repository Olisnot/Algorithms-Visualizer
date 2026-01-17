use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Bar {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[component]
pub fn BarChart(data: ReadSignal<Vec<f64>>, domain: (f64, f64)) -> impl IntoView {
    let w: f64 = 1000.0;
    let h: f64 = 500.0;
    let pad: f64 = 24.0;
    let gap: f64 = 6.0;

    let bars = Memo::new(move |_| {
        let d = data.get();
        if d.is_empty() {
            return Vec::<Bar>::new();
        }

        let (min, max) = domain;

        let min0 = min.min(0.0);
        let max0 = max.max(0.0);
        let range = (max0 - min0).max(1e-9_f64);

        let inner_w = (w - 2.0 * pad).max(1.0_f64);
        let inner_h = (h - 2.0 * pad).max(1.0_f64);

        let n = d.len() as f64;
        let bar_w = ((inner_w - gap * (n - 1.0)) / n).max(1.0);

        let zero_y = pad + ((max0 - 0.0) / range) * inner_h;

        d.iter()
            .enumerate()
            .map(|(i, &v)| {
                let x = pad + i as f64 * (bar_w + gap);

                let bar_h = ((v - 0.0).abs() / range) * inner_h;

                if v >= 0.0 {
                    Bar {
                        x,
                        y: (zero_y - bar_h),
                        w: bar_w,
                        h: bar_h,
                    }
                } else {
                    Bar {
                        x,
                        y: zero_y,
                        w: bar_w,
                        h: bar_h,
                    }
                }
            })
            .collect::<Vec<_>>()
    });

    view! {
      <svg class="tab-content" width=w height=h>
        <g>
          {move || {
            bars.get()
              .into_iter()
              .map(|b| view! {
                <rect
                  x=b.x
                  y=b.y
                  width=b.w
                  height=b.h
                  fill="currentColor"
                  rx="2"
                />
              })
              .collect_view()
          }}
        </g>
      </svg>
    }
}
