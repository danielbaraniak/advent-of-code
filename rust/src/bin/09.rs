use std::vec;

advent_of_code::solution!(9);

const FREE: i32 = -1;

fn segment_to_array(sector_id: usize, size: u32) -> Vec<i32> {
    if sector_id % 2 == 0 {
        vec![(sector_id / 2) as i32; size as usize]
    } else {
        vec![FREE; size as usize]
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut segments: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(i, n)| segment_to_array(i, n))
        .collect();

    let mut left = 0;
    let mut right = segments.len() - 1;

    while left < right {
        while left < segments.len() && segments[left] != FREE {
            left += 1;
        }

        while right > left && segments[right] == FREE {
            right -= 1;
        }

        if left >= right {
            break;
        }

        segments.swap(left, right);
    }

    let checksum: u64 = segments
        .iter()
        .take_while(|&&file_id| file_id != FREE)
        .enumerate()
        .map(|(index, &file_id)| index as u64 * file_id as u64)
        .sum();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let segments = parse(input);

    let mut defragmented_segments = segments.clone();

    for segment in segments.iter().rev().filter(|seg| seg.id.is_some()) {
        let free_space = defragmented_segments
            .iter()
            .enumerate()
            .find(|(_, seg)| seg.id.is_none() && seg.size >= segment.size);

        match free_space {
            None => continue,
            Some((free_idx, free_space)) => {
                let (allocated, remainder) = free_space.allocate(segment).ok()?;
                defragmented_segments[free_idx] = allocated;
                if remainder.size > 0 {
                    defragmented_segments.insert(free_idx + 1, remainder);
                }

                if let Some(idx) = defragmented_segments
                    .iter()
                    .rposition(|seg| seg.id == segment.id)
                {
                    defragmented_segments[idx].id = None;
                }
            }
        }
    }

    let mut checksum = 0;

    defragmented_segments.iter().fold(0, |offset, segment| {
        if let Some(id) = segment.id {
            let start = offset;
            let end = offset + segment.size as u64 - 1;
            checksum += id as u64 * (start + end) * segment.size as u64 / 2;
        }
        offset + segment.size as u64
    });

    Some(checksum)
}

#[derive(Clone)]
struct Segment {
    id: Option<i32>,
    size: u32,
}

fn parse(input: &str) -> Vec<Segment> {
    input
        .trim()
        .char_indices()
        .map(|(i, c)| match i % 2 {
            0 => Segment {
                id: Some((i / 2) as i32),
                size: c.to_digit(10).unwrap(),
            },
            _ => Segment {
                id: None,
                size: c.to_digit(10).unwrap(),
            },
        })
        .collect()
}

#[derive(Debug)]
enum SegmentError {
    InsufficientSpace,
    NotFree,
}

impl Segment {
    fn allocate(&self, new_segment: &Segment) -> Result<(Segment, Segment), SegmentError> {
        match (self.id, self.size >= new_segment.size) {
            (None, true) => Ok((
                Segment {
                    id: new_segment.id,
                    size: new_segment.size,
                },
                Segment {
                    id: None,
                    size: self.size - new_segment.size,
                },
            )),
            (_, true) => Err(SegmentError::NotFree),
            (_, false) => Err(SegmentError::InsufficientSpace),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
