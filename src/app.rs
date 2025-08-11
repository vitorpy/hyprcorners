use crate::config::Config;

use std::time::{Duration, Instant};

use hyprland::data::{CursorPosition, Monitors};
use hyprland::shared::HyprData;
use hyprland::Result;

pub struct App;

impl App {
    pub async fn run(config: Config) -> Result<()> {
        let sticky = config.sticky_timeout.is_some();

        let mut last_switch = None;
        let mut inside_corner = false;

        loop {
            let mut modified_inside_corner = false;

            let cursor_pos = CursorPosition::get_async().await?;
            let (cursor_x, cursor_y) = (cursor_pos.x, cursor_pos.y);

            let monitors = Monitors::get_async().await?;

            for monitor in monitors {
                let monitor_config = match config.get_monitor_config(&monitor.name) {
                    Some(cfg) => cfg,
                    None => continue,
                };

                let (mon_x, mon_y) = (monitor.x as i64, monitor.y as i64);
                let (mon_width, mon_height) = (monitor.width as i64, monitor.height as i64);

                if cursor_x >= mon_x && cursor_x < mon_x + mon_width && 
                   cursor_y >= mon_y && cursor_y < mon_y + mon_height {

                    let relative_x = cursor_x - mon_x;
                    let relative_y = cursor_y - mon_y;

                    let since_last_sticky_switch =
                        last_switch.map(|last_switch| Instant::now().duration_since(last_switch));

                    if !sticky
                        || since_last_sticky_switch.is_none_or(|since_last_sticky_switch| {
                            since_last_sticky_switch.as_millis() >= config.sticky_timeout.unwrap() as u128
                        })
                    {
                        let dispatch_permitted = sticky || !inside_corner;

                        if let Some(ref corner) = monitor_config.top_right {
                            if relative_x > mon_width - corner.radius && relative_y < corner.radius {
                                if dispatch_permitted {
                                    corner.dispatch(&sticky, &mut last_switch).await?;
                                }

                                inside_corner = true;
                                modified_inside_corner = true;
                            }
                        }

                        if let Some(ref corner) = monitor_config.top_left {
                            if relative_x < corner.radius && relative_y < corner.radius {
                                if dispatch_permitted {
                                    corner.dispatch(&sticky, &mut last_switch).await?;
                                }

                                inside_corner = true;
                                modified_inside_corner = true;
                            }
                        }

                        if let Some(ref corner) = monitor_config.bottom_right {
                            if relative_x > mon_width - corner.radius
                                && relative_y > mon_height - corner.radius
                            {
                                if dispatch_permitted {
                                    corner.dispatch(&sticky, &mut last_switch).await?;
                                }

                                inside_corner = true;
                                modified_inside_corner = true;
                            }
                        }

                        if let Some(ref corner) = monitor_config.bottom_left {
                            if relative_x < corner.radius && relative_y > mon_height - corner.radius {
                                if dispatch_permitted {
                                    corner.dispatch(&sticky, &mut last_switch).await?;
                                }

                                inside_corner = true;
                                modified_inside_corner = true;
                            }
                        }

                        if !modified_inside_corner {
                            inside_corner = false;
                        }
                    }
                    
                    break;
                }
            }

            std::thread::sleep(Duration::from_millis(config.timeout));
        }
    }
}