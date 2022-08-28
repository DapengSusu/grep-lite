fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    // 保存已匹配行的行号
    let mut tags: Vec<usize> = vec![];
    // 保存匹配行上下文的若干行信息
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2 * ctx_lines + 1);

            ctx.push(v);
        }
    }

    // 如果没有任何匹配行，提前退出
    if tags.is_empty() {
        return;
    }

    // 在每行上针对每个标记进行检查，检查该行是否在某个匹配行的附近
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if i >= lower_bound && i <= upper_bound {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);

                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;

            println!("{}: {}", line_num, line);
        }
    }
}
