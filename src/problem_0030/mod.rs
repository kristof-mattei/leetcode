use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

fn find_substring(s: &str, words: &[String]) -> Vec<i32> {
    let mut indexes = vec![];

    if words.is_empty() {
        return indexes;
    }

    let word_size = words[0].len();
    let window_size = word_size * words.len();

    let Some(last_split) = s.len().checked_sub(window_size) else {
        return indexes;
    };

    let mut word_set = HashMap::with_capacity(words.len());
    for w in words {
        word_set.entry(&w[..]).and_modify(|c| *c += 1).or_insert(1);
    }
    word_set.shrink_to_fit();

    let mut seen = word_set.keys().map(|k| (*k, 0)).collect::<HashMap<_, _>>();

    for correction_of_1 in 0..word_size.min(last_split + 1) {
        let mut end_of_chunk = window_size + correction_of_1;

        while end_of_chunk <= s.len() {
            let mut word_count = 0;

            while word_count < words.len() {
                // walk backwards from the end of the chunk, checking each word
                let end = end_of_chunk - (word_count * word_size);

                let current = &s[end - word_size..end];

                if let Some(res) = seen.get_mut(current) {
                    *res += 1;
                    if *res > *word_set.get(current).unwrap() {
                        break;
                    }
                    word_count += 1;
                } else {
                    break;
                }
            }

            if word_count == words.len() {
                indexes.push((end_of_chunk - window_size) as i32);
                // if we have a hit, advance by 1 word size
                end_of_chunk += word_size;
            } else {
                // shift by the amount of found words
                // those last words are then the last ones in the chunk we check
                // because we go from last to first. This allows us to focus on new NEW words first
                end_of_chunk = window_size + (end_of_chunk - word_count * word_size);
            }

            seen.values_mut().for_each(|v| *v = 0);
        }
    }

    indexes
}

impl Solution {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        find_substring(&s, &words)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::problem_0030::find_substring;

    #[test]
    fn test() {
        assert_eq!(
            find_substring(
                "barfoothefoobarman",
                &vec!["foo", "bar"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            ),
            vec![0, 9]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            find_substring(
                "wordgoodgoodgoodbestword",
                &vec!["word", "good", "best", "word"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            ),
            ([] as [i32; 0])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            find_substring(
                "barfoofoobarthefoobarman",
                &vec!["bar", "foo", "the"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            ),
            vec![6, 9, 12]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            find_substring(
                "a",
                &vec!["a", "a"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            ),
            ([] as [i32; 0])
        );
    }

    #[test]
    fn test_5() {
        let input = "pjzkrkevzztxductzzxmxsvwjkxpvukmfjywwetvfnujhweiybwvvsrfequzkhossmootkmyxgjgfordrpapjuunmqnxxdrqrfgkrsjqbszgiqlcfnrpjlcwdrvbumtotzylshdvccdmsqoadfrpsvnwpizlwszrtyclhgilklydbmfhuywotjmktnwrfvizvnmfvvqfiokkdprznnnjycttprkxpuykhmpchiksyucbmtabiqkisgbhxngmhezrrqvayfsxauampdpxtafniiwfvdufhtwajrbkxtjzqjnfocdhekumttuqwovfjrgulhekcpjszyynadxhnttgmnxkduqmmyhzfnjhducesctufqbumxbamalqudeibljgbspeotkgvddcwgxidaiqcvgwykhbysjzlzfbupkqunuqtraxrlptivshhbihtsigtpipguhbhctcvubnhqipncyxfjebdnjyetnlnvmuxhzsdahkrscewabejifmxombiamxvauuitoltyymsarqcuuoezcbqpdaprxmsrickwpgwpsoplhugbikbkotzrtqkscekkgwjycfnvwfgdzogjzjvpcvixnsqsxacfwndzvrwrycwxrcismdhqapoojegggkocyrdtkzmiekhxoppctytvphjynrhtcvxcobxbcjjivtfjiwmduhzjokkbctweqtigwfhzorjlkpuuliaipbtfldinyetoybvugevwvhhhweejogrghllsouipabfafcxnhukcbtmxzshoyyufjhzadhrelweszbfgwpkzlwxkogyogutscvuhcllphshivnoteztpxsaoaacgxyaztuixhunrowzljqfqrahosheukhahhbiaxqzfmmwcjxountkevsvpbzjnilwpoermxrtlfroqoclexxisrdhvfsindffslyekrzwzqkpeocilatftymodgztjgybtyheqgcpwogdcjlnlesefgvimwbxcbzvaibspdjnrpqtyeilkcspknyylbwndvkffmzuriilxagyerjptbgeqgebiaqnvdubrtxibhvakcyotkfonmseszhczapxdlauexehhaireihxsplgdgmxfvaevrbadbwjbdrkfbbjjkgcztkcbwagtcnrtqryuqixtzhaakjlurnumzyovawrcjiwabuwretmdamfkxrgqgcdgbrdbnugzecbgyxxdqmisaqcyjkqrntxqmdrczxbebemcblftxplafnyoxqimkhcykwamvdsxjezkpgdpvopddptdfbprjustquhlazkjfluxrzopqdstulybnqvyknrchbphcarknnhhovweaqawdyxsqsqahkepluypwrzjegqtdoxfgzdkydeoxvrfhxusrujnmjzqrrlxglcmkiykldbiasnhrjbjekystzilrwkzhontwmehrfsrzfaqrbbxncphbzuuxeteshyrveamjsfiaharkcqxefghgceeixkdgkuboupxnwhnfigpkwnqdvzlydpidcljmflbccarbiegsmweklwngvygbqpescpeichmfidgsjmkvkofvkuehsmkkbocgejoiqcnafvuokelwuqsgkyoekaroptuvekfvmtxtqshcwsztkrzwrpabqrrhnlerxjojemcxel";
        let words = [
            "dhvf", "sind", "ffsl", "yekr", "zwzq", "kpeo", "cila", "tfty", "modg", "ztjg", "ybty",
            "heqg", "cpwo", "gdcj", "lnle", "sefg", "vimw", "bxcb",
        ];

        assert_eq!(
            find_substring(
                input,
                &words.iter().map(ToString::to_string).collect::<Vec<_>>()
            ),
            vec![935]
        );
    }
}
