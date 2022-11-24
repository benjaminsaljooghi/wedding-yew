use super::*;
use chrono::Datelike;
use chrono::{DateTime, Duration, Utc};
use tracing::{debug, info};

#[derive(PartialEq, Clone, Debug)]
pub enum WeddingDayStatus {
    Coming,
    Today,
    Passed,
}

#[derive(Properties, PartialEq, Debug)]
pub struct WeddingDayProviderProps {
    #[prop_or_default]
    pub children: Children,
    pub wedding_datetime: DateTime<Utc>,
    pub utc_offset: Duration,
}

#[derive(PartialEq, Clone, Debug)]
pub struct WeddingDayInfo {
    pub relative_day_status: WeddingDayStatus,
    pub datetime_str: String,
}

#[function_component(WeddingDayProvider)]
pub fn wedding_day_provider(props: &WeddingDayProviderProps) -> Html {
    debug!(WeddingDayProviderProps = ?props);
    let now = Utc::now();
    let status = get_wedding_day_status(&props.wedding_datetime, &now, &props.utc_offset);
    let with_offset = props.wedding_datetime + props.utc_offset;
    let datetime_str = format!("{} UTC+10", with_offset.format("%d.%m.%Y %-l%p"));

    let wedding_day_info = WeddingDayInfo {
        relative_day_status: status,
        datetime_str,
    };
    debug!(WeddingDayInfo = ?wedding_day_info);
    html! {
        <ContextProvider<WeddingDayInfo> context={wedding_day_info}>
            {for props.children.iter()}
        </ContextProvider<WeddingDayInfo>>
    }
}

pub fn get_wedding_day_status(
    wedding_day: &DateTime<Utc>,
    now: &DateTime<Utc>,
    offset: &Duration,
) -> WeddingDayStatus {
    let wedding_day_with_offset = *wedding_day + *offset;
    let now_with_offset = *now + *offset;

    if wedding_day_with_offset.day() == now_with_offset.day() {
        return WeddingDayStatus::Today;
    }

    if wedding_day_with_offset.timestamp_millis() > now_with_offset.timestamp_millis() {
        return WeddingDayStatus::Coming;
    }
    WeddingDayStatus::Passed
}

#[test]
fn wedding_day_status_should_be_correct() {
    use chrono::TimeZone;
    let wedding_day = Utc.with_ymd_and_hms(2024, 12, 25, 22, 0, 0).unwrap();
    let offset = Duration::seconds(10 * 3600);

    let check_day_today = Utc.with_ymd_and_hms(2024, 12, 25, 15, 0, 0).unwrap();
    let check_day_coming = Utc.with_ymd_and_hms(2024, 12, 25, 13, 0, 0).unwrap();
    let check_day_today_2 = Utc.with_ymd_and_hms(2024, 12, 26, 13, 0, 0).unwrap();
    let check_day_passed = Utc.with_ymd_and_hms(2024, 12, 26, 15, 0, 0).unwrap();

    let today = get_wedding_day_status(&wedding_day, &check_day_today, &offset);
    assert_eq!(today, WeddingDayStatus::Today);

    let today = get_wedding_day_status(&wedding_day, &check_day_today_2, &offset);
    assert_eq!(today, WeddingDayStatus::Today);

    let coming = get_wedding_day_status(&wedding_day, &check_day_coming, &offset);
    assert_eq!(coming, WeddingDayStatus::Coming);

    let passed = get_wedding_day_status(&wedding_day, &check_day_passed, &offset);
    assert_eq!(passed, WeddingDayStatus::Passed);
}
