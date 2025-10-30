pub const SECONDS_IN_MINUTE: u64 = 60;
pub const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;
pub const SECONDS_IN_DAY: u64 = 24 * SECONDS_IN_HOUR;
pub const SECONDS_IN_YEAR: u64 = 365 * SECONDS_IN_DAY;
pub const SECONDS_IN_LEAP_YEAR: u64 = 366 * SECONDS_IN_DAY;

pub const DAYS_IN_MONTH: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const DAYS_IN_MONTH_LEAP_YEAR: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const UTC_OFFSET_SECONDS: u64 = 8 * 60 * 60; // UTC+8 (东八区)

// fn main() {
//     // 示例时间戳
//     let start_timestamp = 1735660700; // 2025年3月1日 00:00:00 UTC
//     let end_timestamp = 1740758500;   // 2025年1月1日 00:00:00 UTC
//     // let number = count_first_of_months(start_timestamp, end_timestamp);

//     // let data = timestamp_to_ymd(1732982400);

//     // println!("number: {:?},{:?},{:?}", data.0,data.1,data.2); // 输出计算结果
//     let mut timestamp = date_to_timestamp(2028,12,16);

//     let data = timestamp_to_ymd(timestamp);
//     println!("{:?}",timestamp);

//     println!("number: {:?},{:?},{:?}", data.0,data.1,data.2); // 输出计算结果

//     let purchse = generate_release_timestamps(timestamp);

//     println!("purchse: {:?}", purchse);
//     for iter in purchse.iter(){

//         println!("purchse: {:?}", iter);
//         let data = timestamp_to_ymd(*iter);
//         println!("number: {:?},{:?},{:?}", data.0,data.1,data.2);
//     }

// }

//计算MONTHLY_TIMESTAMP_LIST
pub fn count_first_of_months(
    start_timestamp: u64,
    end_timestamp: u64,
    MONTHLY_TIMESTAMP_LIST: [u64; 12],
) -> u64 {
    let mut count = 0;

    // 遍历月份时间戳，统计在范围内的1号日期
    for &timestamp in MONTHLY_TIMESTAMP_LIST.iter() {
        // 如果该月1号的时间戳在给定的时间范围内
        if timestamp >= start_timestamp && timestamp <= end_timestamp {
            count += 1;
        }
    }
    count
}

fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

//参数为utc+0时间，得出结果为utc+8 日期
pub fn timestamp_to_ymd(mut unix_timestamp: u64) -> (u64, u64, u64) {
    let mut timestamp = unix_timestamp;

    let mut year = 1970;

    timestamp += UTC_OFFSET_SECONDS;
    // 减去年份的秒数，直到找到具体年份
    while timestamp >= 0 {
        let year_seconds = if is_leap_year(year) {
            SECONDS_IN_LEAP_YEAR
        } else {
            SECONDS_IN_YEAR
        };

        if timestamp >= year_seconds {
            timestamp -= year_seconds;
            year += 1;
        } else {
            break;
        }
    }

    // 找到月份
    let mut month = 0;
    let days_in_month = if is_leap_year(year) {
        &DAYS_IN_MONTH_LEAP_YEAR
    } else {
        &DAYS_IN_MONTH
    };

    while timestamp >= 0 {
        let month_days = days_in_month[month] * SECONDS_IN_DAY;
        if timestamp >= month_days {
            timestamp -= month_days;
            month += 1;
        } else {
            break;
        }
    }

    let day = (timestamp / SECONDS_IN_DAY) + 1;

    (year, (month + 1) as u64, day) // 返回年月日
}

// 日期转换为 Unix 时间戳， 参数为utc+8 时间，得出utc+0 时间戳
fn date_to_timestamp(year: u64, month: u64, day: u64) -> u64 {
    // const SECONDS_IN_DAY: u64 = 24 * 60 * 60;
    const DAYS_IN_MONTH: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut timestamp = 0;

    // 加上前面的年数
    for y in 1970..year {
        timestamp += if is_leap_year(y) {
            SECONDS_IN_DAY * 366
        } else {
            SECONDS_IN_DAY * 365
        };
    }

    // 加上前面的月数
    for m in 1..month {
        timestamp += SECONDS_IN_DAY * if m == 2 && is_leap_year(year) {
            29
        } else {
            DAYS_IN_MONTH[(m - 1) as usize]
        };
    }

    // 加上天数
    timestamp += SECONDS_IN_DAY * (day - 1);
    timestamp -= UTC_OFFSET_SECONDS;

    timestamp
}

/// 根据购买时间戳生成 12 个释放时间戳
pub fn generate_release_timestamps(purchase_timestamp: u64) -> ([u64; 12], u64) {
    let (year, month, day) = timestamp_to_ymd(purchase_timestamp);

    let end_buy_timedata = if day <= 15 {
        (year, month, 16)
    } else {
        if month == 12 {
            (year + 1, 1, 1)
        } else {
            (year, month + 1, 1)
        }
    };
    let end_buy_time =
        date_to_timestamp(end_buy_timedata.0, end_buy_timedata.1, end_buy_timedata.2);

    // 确定释放的起始日期和截止日期
    let (mut start_year, mut start_month, mut start_day) = if day <= 15 {
        // 如果购买日期在1号到15号之间，截止时间是下个月的16号
        if month == 12 {
            (year + 1, 1, 16)
        } else {
            (year, month + 1, 16)
        }
    } else {
        // 如果购买日期在16号到月底之间，截止时间是下下个月的1号
        if month == 12 {
            (year + 1, 2, 1)
        } else {
            (year, month + 2, 1)
        }
    };

    // 定义一个定长数组来存储12个月的释放时间戳
    let mut release_timestamps = [0u64; 12];

    // 生成连续 12 个月的释放时间戳
    for i in 0..12 {
        // 计算并保存释放时间戳
        release_timestamps[i] = date_to_timestamp(start_year, start_month, start_day);

        // 更新下个月的日期
        if start_month == 12 {
            start_month = 1;
            start_year += 1;
        } else {
            start_month += 1;
        }

        // // 如果是16号到月底购买，则下次的截止日期是下下个月的1号
        // if start_day == 16 {
        //     start_day = 1;
        // } else {
        //     start_day = 16;
        // }
    }

    // 返回释放时间戳数组和第一个截止日期的时间戳

    (release_timestamps, end_buy_time)
}

/// 根据购买时间戳生成 12 个释放时间戳
pub fn test_generate_release_timestamps(purchase_timestamp: u64) -> ([u64; 12], u64) {
    let end_buy_time = purchase_timestamp + 600;

    // 定义一个定长数组来存储12个月的释放时间戳
    let mut release_timestamps = [0u64; 12];

    // 生成连续 12 个月的释放时间戳
    for i in 0..12 {
        // 计算并保存释放时间戳
        release_timestamps[i] = end_buy_time + (i as u64 + 1) * 600;

        // 更新下个月的日期
    }

    // 返回释放时间戳数组和第一个截止日期的时间戳
    (release_timestamps, end_buy_time)
}
