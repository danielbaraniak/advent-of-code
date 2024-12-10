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

        while right > 0 && segments[right] == FREE {
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

    for segment in segments.iter().rev().filter(|seg| seg.id != FREE) {
        let free_space = defragmented_segments
            .iter()
            .enumerate()
            .find(|(_, seg)| seg.id == FREE && seg.size >= segment.size);

        match free_space {
            None => continue,
            Some((free_idx, free_space)) => {
                let (allocated, remainder) = free_space.allocate(segment).ok()?;
                defragmented_segments[free_idx] = allocated;
                if remainder.size > 0 {
                    defragmented_segments.insert(free_idx + 1, remainder);
                }

                if let Some((idx, _)) = defragmented_segments
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, seg)| seg.id == segment.id)
                {
                    defragmented_segments[idx].id = FREE;
                }
            }
        }
    }

    let mut offset = 0;
    let mut checksum = 0;

    for segment in defragmented_segments.iter() {
        if segment.id != FREE {
            let start = offset;
            let end = offset + segment.size as u64 - 1;
            checksum += segment.id as u64 * (start + end) * segment.size as u64 / 2;
        }
        offset += segment.size as u64;
    }

    Some(checksum)
}

#[derive(Clone)]
struct Segment {
    id: i32,
    size: u32,
}

fn parse(input: &str) -> Vec<Segment> {
    input
        .trim()
        .char_indices()
        .map(|(i, c)| {
            if i % 2 == 0 {
                Segment {
                    id: (i / 2) as i32,
                    size: c.to_digit(10).unwrap(),
                }
            } else {
                Segment {
                    id: FREE,
                    size: c.to_digit(10).unwrap(),
                }
            }
        })
        .collect()
}

#[derive(Debug)]
enum SegmentError {
    InsufficientSpace,
}
impl Segment {
    fn allocate(&self, new_segment: &Segment) -> Result<(Segment, Segment), SegmentError> {
        if self.id == FREE && self.size >= new_segment.size {
            Ok((
                Segment {
                    id: new_segment.id,
                    size: new_segment.size,
                },
                Segment {
                    id: FREE,
                    size: self.size - new_segment.size,
                },
            ))
        } else {
            Err(SegmentError::InsufficientSpace)
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
