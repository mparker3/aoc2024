use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Segment {
    start: usize,
    len: usize,
    is_file: bool,
    file_id: Option<usize>,
}

fn main() {
    let input = helpers::get_input("sample.txt");
    let input = input.trim();
    
    let mut pos = 0;
    let mut segments = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;
    
    for (i, c) in input.chars().enumerate() {
        let len = c.to_digit(10).unwrap() as usize;
        
        segments.push(Segment {
            start: pos,
            len,
            is_file,
            file_id: if is_file { Some(file_id) } else { None },
        });
        
        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
        pos += len;
    }
    
    let mut file_ids: Vec<usize> = segments.iter()
        .filter_map(|s| s.file_id)
        .collect();
    file_ids.sort_by(|a, b| b.cmp(a));  
    visualize(&segments, "Initial state");
    
    for &id in &file_ids {
        let (file_idx, file) = segments.iter()
            .enumerate()
            .find(|(_, s)| s.file_id == Some(id))
            .unwrap();
        let file_len = file.len;
        
        let mut best_space_idx = None;
        let mut best_space_start = file.start;
        
        for (i, seg) in segments.iter().enumerate() {
            if !seg.is_file && seg.len >= file_len && seg.start < best_space_start {
                best_space_idx = Some(i);
                best_space_start = seg.start;
            }
        }
        
        if let Some(space_idx) = best_space_idx {
            let space = segments[space_idx].clone();
            
            segments.remove(file_idx);
            
            segments.remove(if space_idx < file_idx { space_idx } else { space_idx - 1 });
            
            let new_file = Segment {
                start: space.start,
                len: file_len,
                is_file: true,
                file_id: Some(id),
            };
            segments.insert(space_idx, new_file);
            
            if space.len > file_len {
                let new_space = Segment {
                    start: space.start + file_len,
                    len: space.len - file_len,
                    is_file: false,
                    file_id: None,
                };
                segments.insert(space_idx + 1, new_space);
            }
        }
    }
    
    let mut checksum = 0i64;
    for seg in &segments {
        if let Some(id) = seg.file_id {
            for pos in seg.start..seg.start + seg.len {
                checksum += (pos as i64) * (id as i64);
            }
        }
    }
    visualize(&segments, "Final state");
    
    println!("Checksum: {}", checksum);
}

fn visualize(segments: &[Segment], label: &str) {
    println!("\n=== {} ===", label);
    
    let disk_size = segments.iter()
        .map(|s| s.start + s.len)
        .max()
        .unwrap_or(0);
    
    for i in 0..disk_size {
        print!("{}", i % 10);
    }
    println!();
    
    let mut disk = vec!['.'; disk_size];
    for seg in segments {
        if let Some(id) = seg.file_id {
            for pos in seg.start..seg.start + seg.len {
                disk[pos] = char::from_digit(id as u32, 10).unwrap_or('?');
            }
        }
    }
    println!("{}", disk.iter().collect::<String>());
}