using System.Collections.Generic;
using System.Linq;
using OpenUtau.Api;
using OpenUtau.Core.Ustx;

namespace OpenUtau.Plugin.Builtin {
    [Phonemizer("Japanese VCV Phonemizer (legacy)", "JA VCV", language: "JA")]
    public class JapaneseVCVPhonemizer : Phonemizer {

        static readonly string[] vowels = new string[] {
            "a=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,a",
            "e=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,e",
            "i=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,i",
            "o=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,o",
            "n=,n",
            "u=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,u",
            "N=,ng",
        };

        static readonly Dictionary<string, string> vowelLookup;

        static JapaneseVCVPhonemizer() {

            vowelLookup = vowels.ToList()
                .SelectMany(line => {
                    var parts = line.Split('=');
                    return parts[1].Split(',').Select(cv => (cv, parts[0]));
                })
                .ToDictionary(t => t.Item1, t => t.Item2);
        }

        private USinger singer;

        public override void SetSinger(USinger singer) => this.singer = singer;

        public override Result Process(Note[] notes, Note? prev, Note? next, Note? prevNeighbour, Note? nextNeighbour, Note[] prevNeighbours) {
            var note = notes[0];
            var currentLyric = note.lyric.Normalize();

            if (!string.IsNullOrEmpty(note.phoneticHint)) {

                if (CheckOtoUntilHit(new string[] { note.phoneticHint.Normalize() }, note, out var ph)) {
                    return new Result {
                        phonemes = new Phoneme[] {
                            new Phoneme {
                                phoneme = ph.Alias,
                            }
                        },
                    };
                }
            }

            string[] tests = new string[] { $"- {currentLyric}" , currentLyric};
            if (prevNeighbour != null) {

                var prevLyric = prevNeighbour.Value.lyric.Normalize();
                if (!string.IsNullOrEmpty(prevNeighbour.Value.phoneticHint)) {
                    prevLyric = prevNeighbour.Value.phoneticHint.Normalize();
                }

                var unicode = ToUnicodeElements(prevLyric);

                if (vowelLookup.TryGetValue(unicode.LastOrDefault() ?? string.Empty, out var vow)) {

                    tests = new string[] { $"{vow} {currentLyric}", $"* {currentLyric}", currentLyric, $"- {currentLyric}" };
                }
            }
            if (CheckOtoUntilHit(tests, note, out var oto)) {
                return new Result {
                    phonemes = new Phoneme[] {
                        new Phoneme {
                            phoneme = oto.Alias,
                        }
                    },
                };
            }
            return new Result {
                phonemes = new Phoneme[] {
                    new Phoneme {
                        phoneme = currentLyric,
                    }
                },
            };
        }

        private bool CheckOtoUntilHit(string[] input, Note note, out UOto oto) {
            oto = default;
            var attr = note.phonemeAttributes?.FirstOrDefault(attr => attr.index == 0) ?? default;
            string color = attr.voiceColor ?? GetParentVoiceColor();
            int shift = attr.toneShift ?? GetParentToneShift();
            int? alt = attr.alternate ?? GetParentAlternate();

            var otos = new List<UOto>();
            foreach (string test in input) {
                if (singer.TryGetMappedOto(test + alt, note.tone + shift, color, out var otoAlt)) {
                    otos.Add(otoAlt);
                } else if (singer.TryGetMappedOto(test, note.tone + shift, color, out var otoCandidacy)) {
                    otos.Add(otoCandidacy);
                }
            }

            if (otos.Count > 0) {
                oto = otos.FirstOrDefault(oto => oto.IsColorMatch(color));
                if (oto == null) {
                    oto = otos.First();
                }
                return true;
            }
            return false;
        }
    }
}
