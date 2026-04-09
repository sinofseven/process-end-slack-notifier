use chrono::{FixedOffset, Utc};
use serde::Serialize;
use slack_morphism::blocks::{
    SlackBlock, SlackBlockMarkDownText, SlackDividerBlock, SlackSectionBlock,
};

#[derive(Debug, Clone, Serialize)]
struct Payload {
    blocks: Vec<SlackBlock>,
}

const OFFSET_JST: FixedOffset = FixedOffset::east_opt(9 * 3600).unwrap();

pub fn send_slack(url: &String, process: &crate::models::processes::Process) -> Result<(), String> {
    let payload = build_blocks(process);
    ureq::post(url)
        .send_json(&payload)
        .map(|_| ())
        .map_err(|e| format!("failed to call incoming webhook: {e}"))
}

fn jst_now_iso_format() -> String {
    let dt_utc = Utc::now();
    let dt_jst = dt_utc.with_timezone(&OFFSET_JST);

    dt_jst.to_rfc3339()
}

fn generate_section_block(message: String) -> SlackBlock {
    SlackBlock::Section(
        SlackSectionBlock::new().with_text(SlackBlockMarkDownText::new(message).into()),
    )
}

fn build_blocks(process: &crate::models::processes::Process) -> Payload {
    let mut blocks: Vec<SlackBlock> = vec![
        generate_section_block(format!("<!channel> `{}`", jst_now_iso_format())),
        SlackBlock::Divider(SlackDividerBlock::new()),
        generate_section_block("Process Terminated.".to_string()),
        generate_section_block(format!("*PID*: `{}`", process.pid)),
        generate_section_block(format!("*Command*: `{}`", process.command)),
        generate_section_block(format!("*Cwd*: `{}`", process.cwd)),
    ];

    if let Some(memo) = &process.memo {
        blocks.push(generate_section_block(format!("*Memo*: `{}`", process.cwd)))
    }

    Payload { blocks }
}
