using System.Collections.Generic;
using System.Linq;
using OpenUtau.Api;
using Pinyin;

namespace OpenUtau.Plugin.Builtin {

    [Phonemizer("Cantonese CVVC Phonemizer", "ZH-YUE CVVC", "Lotte V", language: "ZH-YUE")]
    public class CantoneseCVVCPhonemizer : ChineseCVVCPhonemizer {
        protected override string[] Romanize(IEnumerable<string> lyrics) {
            return Pinyin.Jyutping.Instance.HanziToPinyin(lyrics.ToList(), CanTone.Style.NORMAL, Pinyin.Error.Default).Select(res => res.pinyin).ToArray();
        }
    }
}
