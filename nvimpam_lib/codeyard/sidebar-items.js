initSidebarItems({"fn":[["check_rest","Helper function used by parse_str3."],["create_card_data5","Use FoldExt to creat the card data. Seems slower than the direct way.  Might not be correct, see this comment."],["parse_str2","parse_str seems to largely dominate the benchmark for create_card_data, this might be a faster alternative. Needs real benchmarks!"],["parse_str3","parse_str seems to largely dominate the benchmark for create_card_data, this might be a faster alternative. Needs real benchmarks!"],["parse_str4","parse_str seems to largely dominate the benchmark for create_card_data, this might be a faster alternative. Needs real benchmarks!"]],"struct":[["Fold","Structure to hold fold data"],["Folds","Structure holding the original iterator I (the Vec in nvimpam) and the state that needs saving between next() calls, ncard. This is the next fold to return without touching the original iterator. That can happen if we iterated \"too far\" while looking for the next card type after a comment, in which case ncard will be Some(Comment) and the iterator will continue after the comment."]],"trait":[["FoldExt","Trait that creates an iterator adaptor to contract folding data."]]});