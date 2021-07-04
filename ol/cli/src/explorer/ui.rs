use crate::explorer::App;
use crate::{cache::Vitals};
use diem_types::{account_address::AccountAddress, account_state::AccountState};
use std::convert::TryFrom;
use tui::layout::Alignment;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, /*Line,*/ Map, MapResolution},
    widgets::{Block, Borders, Cell, LineGauge, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};

/// draw app
pub fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_explorer_tab(f, app, chunks[1]),
        1 => draw_pilot_tab(f, app, chunks[1]),
        2 => draw_network_tab(f, app, chunks[1]),
        3 => draw_txs_tab(f, app, chunks[1]),
        4 => draw_coins_tab(f, app, chunks[1]),
        _ => {}
    };
}

///draw first tab
fn draw_pilot_tab<B>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect)
where
    B: Backend,
{

    let node_home = app.node.conf.clone().workspace.node_home.clone();
    let cached_vitals = Vitals::read_json(&node_home);

    let status_webserver = if cached_vitals.items.web_running {
        "web monitor is serving on port 3030"
    } else {
        "web monitor is NOT SERVING"
    };
    let mut status_db_bootstrapped = "DiemDB is NOT BOOTSTRAPPED";
    
    let status_file = if cached_vitals.items.db_files_exist {
        if cached_vitals.items.db_restored {
            status_db_bootstrapped = "DiemDB is bootstrapped."
        }
        "DB files exist".to_owned()
    } else {
        format!("DB files do NOT EXIST {:?}", app.node.conf.workspace.db_path).to_owned()
    };
    let text = vec![
        Spans::from(vec![
            Span::from("\n WebServer "),
            Span::raw(status_webserver),
        ]),
        Spans::from(vec![
            Span::from("\n Files Check: "),
            Span::raw(status_file),
        ]),
        Spans::from(vec![
            Span::raw("\n DB Checks: "),
            Span::raw(status_db_bootstrapped),
        ]),
        Spans::from(vec![
            Span::raw("\n Validator Check: "),
            Span::raw(if cached_vitals.items.validator_set {
                "Account is in validator set"
            } else {
                "Account is NOT in validator set"
            }),
        ]),
        Spans::from(vec![
            Span::raw("\n Node Checks: "),
            Span::raw(if cached_vitals.items.node_running {
                "Node is running"
            } else {
                "Node is NOT running"
            }),
        ]),
        Spans::from(vec![
            Span::raw("\n Miner Checks: "),
            Span::raw(if cached_vitals.items.miner_running  {
                "Miner is running"
            } else {
                "Miner is NOT running"
            }),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" Status ", Style::default()));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

///draw first tab
fn draw_explorer_tab<B>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(7),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_chain_info(f, app, chunks[0]);
    draw_validator_list(f, app, chunks[1]);
    draw_parameters(f, app, chunks[2]);
}

/// draw chain info in first tab
fn draw_chain_info<B>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(1),
                //Constraint::Length(1),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);

    let block = Block::default().borders(Borders::ALL).title(" Overview ");
    f.render_widget(block, area);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let cs = &app.chain_state;
    let paragraph = Paragraph::new(format!(
        "{}",
        match cs {
            Some(cv) => cv.epoch,
            None => 0,
        }
    ))
    .style(Style::default().add_modifier(Modifier::BOLD))
    .block(Block::default().borders(Borders::ALL).title(" Epoch "))
    .alignment(Alignment::Center);
    f.render_widget(paragraph, columns[0]);

    let paragraph = Paragraph::new(format!(
        "{}",
        match cs {
            Some(cv) => cv.height,
            None => 0,
        }
    ))
    .style(Style::default().add_modifier(Modifier::BOLD))
    .block(Block::default().borders(Borders::ALL).title(" Version "))
    .alignment(Alignment::Center);
    f.render_widget(paragraph, columns[1]);

    let paragraph = Paragraph::new(format!(
        "{}",
        match cs {
            Some(cv) => cv.validator_count,
            None => 0,
        }
    ))
    .style(Style::default().add_modifier(Modifier::BOLD))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Validator Count "),
    )
    .alignment(Alignment::Center);
    f.render_widget(paragraph, columns[2]);

    let paragraph = Paragraph::new(format!(
        "{}",
        match cs {
            Some(cv) => cv.total_supply,
            None => 0,
        }
    ))
    .style(Style::default().add_modifier(Modifier::BOLD))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Total Supply "),
    )
    .alignment(Alignment::Center);
    f.render_widget(paragraph, columns[3]);

    let line_gauge = LineGauge::default()
        .block(Block::default().title("Epoch Process: "))
        .gauge_style(Style::default().fg(Color::Green))
        .line_set(if app.enhanced_graphics {
            symbols::line::THICK
        } else {
            symbols::line::NORMAL
        })
        .ratio(app.progress);
    f.render_widget(line_gauge, chunks[1]);
}

/// draw validator list in first tab
fn draw_validator_list<B>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect)
where
    B: Backend,
{
    let up_style = Style::default();
    let failure_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
    let rows = app.validators.iter().map(|s| {
        let style = if s.voting_power > 0 {
            up_style
        } else {
            failure_style
        };
        Row::new(vec![
            s.account_address.to_owned(),
            s.voting_power.to_string(),
            s.epochs_since_last_account_creation.to_string(),
            s.tower_epoch.to_string(),
            s.tower_height.to_string(),
            s.count_proofs_in_epoch.to_string(),
        ])
        .style(style)
    });
    let table = Table::new(rows)
        .header(
            Row::new(vec![
                "VALIDATOR",
                "VOTING POWER",
                "START EPOCH",
                "TOWER EPOCH",
                "TOWER HEIGHT",
                "PROOFS",
            ])
            .style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ), //.bottom_margin(1),
        )
        .block(Block::default().title(" Validators ").borders(Borders::ALL))
        .widths(&[
            Constraint::Ratio(20, 30),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(15),
        ]);
    f.render_widget(table, area);
}

/// draw parameters
fn draw_parameters<B>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect)
where
    B: Backend,
{
    let metadata = app.node.client.get_metadata();
    if metadata.is_ok() {
        let meta = metadata.unwrap();
        let text = vec![
            Spans::from(vec![
                Span::from("Libra Version: "),
                Span::styled(
                    format!("{}", meta.diem_version.unwrap()),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw("    Chain ID: "),
                Span::styled(
                    format!("{}", meta.chain_id),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Spans::from(vec![
                Span::from("Version: "),
                Span::styled(
                    format!("{}", meta.version),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Spans::from(vec![
                Span::raw("Timestamp: "),
                Span::styled(
                    format!("{}", meta.timestamp),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Spans::from(vec![
                Span::raw(" Root Hash: "),
                Span::raw(meta.accumulator_root_hash.to_hex()),
            ]),
            Spans::from(format!(
                "Allow Publish Module: {}",
                meta.module_publishing_allowed.unwrap()
            )),
        ];
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(" Parameters ", Style::default()));
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}

/// draw second tab
fn draw_network_tab<B>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);
    let up_style = Style::default();
    let failure_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
    let rows = app.validators.iter().map(|s| {
        let style = if s.voting_power > 0 {
            up_style
        } else {
            failure_style
        };
        Row::new(vec![
            s.account_address.as_str(),
            s.full_node_ip.as_str(),
            "Up",
        ])
        .style(style)
    });
    let table = Table::new(rows)
        .header(
            Row::new(vec!["Server", "Address", "Status"])
                .style(Style::default().fg(Color::Green))
                .bottom_margin(1),
        )
        .block(Block::default().title(" Full Nodes ").borders(Borders::ALL))
        .widths(&[Constraint::Length(15), Constraint::Length(40)]);
    f.render_widget(table, chunks[0]);

    let map = Canvas::default()
        .block(
            Block::default()
                .title(" Peers In The World ")
                .borders(Borders::ALL),
        )
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.layer();
            // ctx.draw(&Rectangle {
            //     x: 0.0,
            //     y: 30.0,
            //     width: 10.0,
            //     height: 10.0,
            //     color: Color::Yellow,
            // });
            // Connect servers with line
            // for (i, s1) in app.servers.iter().enumerate() {
            //     for s2 in &app.servers[i + 1..] {
            //         ctx.draw(&Line {
            //             x1: s1.coords.1,
            //             y1: s1.coords.0,
            //             x2: s2.coords.1,
            //             y2: s2.coords.0,
            //             color: Color::Yellow,
            //         });
            //     }
            // }
            for server in &app.servers {
                let color = if server.status == "Up" {
                    Color::Green
                } else {
                    Color::Red
                };
                ctx.print(server.coords.1, server.coords.0, "X", color);
            }
        })
        .marker(if app.enhanced_graphics {
            symbols::Marker::Braille
        } else {
            symbols::Marker::Dot
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(map, chunks[1]);
}

/// draw third tab
fn draw_coins_tab<B>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect)
where
    B: Backend,
{
    let mut items: Vec<Row<'_>> = vec![];
    let (blob, _version) = 
        match app.node.client.get_account_state_blob(&AccountAddress::ZERO) {
            Ok(t) => t,
            Err(_) => (None, 0),
    };
    if let Some(account_blob) = blob {
        let account_state = AccountState::try_from(&account_blob).unwrap();
        items = account_state
            .get_registered_currency_info_resources()
            .unwrap()
            .iter()
            .map(|c| {
                let cells = vec![
                    Cell::from(Span::raw(format!("{}", c.currency_code()))),
                    Cell::from(Span::raw(format!("{:?}", c.total_value()))),
                    Cell::from(Span::raw(format!("{:?}", c.fractional_part()))),
                    Cell::from(Span::raw(format!("{:?}", c.scaling_factor()))),
                    Cell::from(Span::raw(format!("{:?}", c.exchange_rate()))),
                ];
                Row::new(cells)
            })
            .collect();
    }
    let table = Table::new(items)
        .header(
            Row::new(vec![
                "Coin",
                "Total Value",
                "Fractional Part",
                "Scaling Factor",
                "Change Rate",
            ])
            .style(Style::default().fg(Color::Green)),
        )
        .block(Block::default().title(" Coins ").borders(Borders::ALL))
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(25),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Ratio(1, 3),
        ]);
    f.render_widget(table, area);
}

/// draw txs tab
fn draw_txs_tab<B>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect)
where
    B: Backend,
{
    let items: Vec<Row<'_>> = app
        .txs
        .iter()
        .map(|c| {
            let cells = vec![
                Cell::from(Span::raw(format!("{}", c.version))),
                Cell::from(Span::raw(format!("{}", c.hash))),
                Cell::from(Span::raw(format!("{:?}", c.gas_used))),
                Cell::from(Span::raw(format!("{:?}", c.vm_status))),
                Cell::from(Span::raw(format!("{:?}", c.transaction))),
            ];
            Row::new(cells)
        })
        .collect();
    let table = Table::new(items)
        .header(
            Row::new(vec!["Version", "Hash", "Gas", "Status", "Type", "Body"])
                .style(Style::default().fg(Color::Green)),
        )
        .block(
            Block::default()
                .title(" Transactions ")
                .borders(Borders::ALL),
        )
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(25),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Ratio(1, 3),
        ]);
    f.render_widget(table, area);
}
