using System;
using System.Collections.Generic;
using System.Linq;
using OpenUtau.Api;
using OpenUtau.Core.Ustx;
using Serilog;

namespace OpenUtau.Plugin.Builtin {
    [Phonemizer("Japanese CVVC Phonemizer (legacy)", "JA CVVC", "TUBS",language:"JA")]
    public class JapaneseCVVCPhonemizer : Phonemizer {
        static readonly string[] plainVowels = new string[] {"","","","","","","",""};
        static readonly string[] nonVowels = new string[]{"","","R","-","k","ky","g","gy",
                                                           "s","sh","z","j","t","ch","ty","ts",
                                                           "d","dy","n","ny","h","hy","f","b",
                                                           "by","p","py","m","my","y","r","4",
                                                           "ry","w","v","ng","l","","B", "H",
        };

        static readonly string[] vowels = new string[] {
            "a=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,",
            "e=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,",
            "i=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,",
            "o=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,",
            "n=",
            "u=,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,",
            "N=",
            "=",
        };

        static readonly string[] consonants = new string[] {
            "ch=,,,,",
            "gy=,,,,",
            "ts=,,,,",
            "ty=,,,,",
            "py=,,,,",
            "ry=,,,,",
            "ly=,,,,",
            "ny=,,,,",
            "r=,,,,",
            "hy=,,,,",
            "dy=,,,,",
            "by=,,,,",
            "b=,,,,",
            "d=,,,,",
            "g=,,,,",
            "f=,,,,",
            "h=,,,,",
            "k=,,,,",
            "j=,,,,,,,,,",
            "m=,,,,",
            "n=,,,,",
            "p=,,,,",
            "s=,,,,",
            "sh=,,,,",
            "t=,,,,",
            "v=,,,,,",
            "ky=,,,,",
            "w=,,,,,,,,,",
            "y=,,,,",
            "z=,,,,",
            "dz=,",
            "my=,,,,",
            "ng=,,,,,,,,,,,,,,,,,",
            "l=,,,",
            "=,,,,,,,",
        };

        static readonly string[] substitution = new string[] {
            "ty,ch,ts=t", "j,dy=d", "gy=g", "ky=k", "py=p", "ny=n", "ry=r", "my=m", "hy,f=h", "by,v=b", "dz=z", "l=r", "ly=l"
        };

        static readonly Dictionary<string, string> vowelLookup;
        static readonly Dictionary<string, string> consonantLookup;
        static readonly Dictionary<string, string> substituteLookup;

        static JapaneseCVVCPhonemizer() {
            vowelLookup = vowels.ToList()
                .SelectMany(line => {
                    var parts = line.Split('=');
                    return parts[1].Split(',').Select(cv => (cv, parts[0]));
                })
                .ToDictionary(t => t.Item1, t => t.Item2);
            consonantLookup = consonants.ToList()
                .SelectMany(line => {
                    var parts = line.Split('=');
                    return parts[1].Split(',').Select(cv => (cv, parts[0]));
                })
                .ToDictionary(t => t.Item1, t => t.Item2);
            substituteLookup = substitution.ToList()
                .SelectMany(line => {
                    var parts = line.Split('=');
                    return parts[0].Split(',').Select(orig => (orig, parts[1]));
                })
                .ToDictionary(t => t.Item1, t => t.Item2);
        }

        private USinger singer;
        public override void SetSinger(USinger singer) => this.singer = singer;

        private bool checkOtoUntilHit(string[] input, Note note, out UOto oto) {
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

        private bool checkOtoUntilHitVc(string[] input, Note note, out UOto oto) {
            oto = default;
            var attr = note.phonemeAttributes?.FirstOrDefault(attr => attr.index == 1) ?? default;
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
                if (oto != null) {
                    return true;
                }
            }
            return false;
        }

        public override Result Process(Note[] notes, Note? prev, Note? next, Note? prevNeighbour, Note? nextNeighbour, Note[] prevNeighbours) {
            var note = notes[0];
            var currentLyric = note.lyric.Normalize();
            if (!string.IsNullOrEmpty(note.phoneticHint)) {
                currentLyric = note.phoneticHint.Normalize();
            }
            var originalCurrentLyric = currentLyric;
            var cfLyric = $"* {currentLyric}";
            var attr0 = note.phonemeAttributes?.FirstOrDefault(attr => attr.index == 0) ?? default;
            var attr1 = note.phonemeAttributes?.FirstOrDefault(attr => attr.index == 1) ?? default;

            if (!string.IsNullOrEmpty(note.phoneticHint)) {
                string[] tests = new string[] { currentLyric };

                if (checkOtoUntilHit(tests, note, out var oto)) {
                    currentLyric = oto.Alias;
                }
            } else if (prevNeighbour == null) {

                var initial = $"- {currentLyric}";
                string[] tests = new string[] { initial, currentLyric };

                if (checkOtoUntilHit(tests, note, out var oto)) {
                    currentLyric = oto.Alias;
                }
            } else if (plainVowels.Contains(currentLyric) || nonVowels.Contains(currentLyric)) {
                var prevLyric = prevNeighbour.Value.lyric.Normalize();
                if (!string.IsNullOrEmpty(prevNeighbour.Value.phoneticHint)) {
                    prevLyric = prevNeighbour.Value.phoneticHint.Normalize();
                }

                if (vowelLookup.TryGetValue(prevLyric.LastOrDefault().ToString() ?? string.Empty, out var vow)) {
                    var vowLyric = $"{vow} {currentLyric}";

                    string[] tests = new string[] {vowLyric, cfLyric, currentLyric};
                    if (checkOtoUntilHit(tests, note, out var oto)){
                        currentLyric = oto.Alias;
                    }
                }
            } else {
                string[] tests = new string[] {cfLyric, currentLyric};
                if (checkOtoUntilHit(tests, note, out var oto)){
                    currentLyric = oto.Alias;
                }
            }

            if (nextNeighbour != null && string.IsNullOrEmpty(nextNeighbour.Value.phoneticHint)) {
                var nextLyric = nextNeighbour.Value.lyric.Normalize();

                if (nextLyric.Length == 1 && plainVowels.Contains(nextLyric)) {
                    return new Result {
                        phonemes = new Phoneme[] {
                            new Phoneme() {
                                phoneme = currentLyric,
                            }
                        },
                    };
                }

                var vowel = "";
                if (vowelLookup.TryGetValue(originalCurrentLyric.LastOrDefault().ToString() ?? string.Empty, out var vow)) {
                    vowel = vow;
                }

                var consonant = "";
                if (consonantLookup.TryGetValue(nextLyric.FirstOrDefault().ToString() ?? string.Empty, out var con) || (nextLyric.Length >= 2 && consonantLookup.TryGetValue(nextLyric.Substring(0, 2), out con))) {
                    consonant = con;
                }

                if (consonant == "") {
                    return new Result {
                        phonemes = new Phoneme[] {
                            new Phoneme() {
                                phoneme = currentLyric,
                            }
                        },
                    };
                }

                var vcPhoneme = $"{vowel} {consonant}";
                var vcPhonemes = new string[] {vcPhoneme, ""};

                if (substituteLookup.TryGetValue(consonant ?? string.Empty, out con)){
                        vcPhonemes[1] = $"{vowel} {con}";
                }

                if (checkOtoUntilHitVc(vcPhonemes, note, out var oto1)) {
                    vcPhoneme = oto1.Alias;
                } else {
                    return new Result {
                        phonemes = new Phoneme[] {
                            new Phoneme() {
                                phoneme = currentLyric,
                            }
                        },
                    };
                }

                int totalDuration = notes.Sum(n => n.duration);
                int vcLength = 120;
                var nextAttr = nextNeighbour.Value.phonemeAttributes?.FirstOrDefault(attr => attr.index == 0) ?? default;
                if (singer.TryGetMappedOto(nextLyric, nextNeighbour.Value.tone + (nextAttr.toneShift ?? GetParentToneShift()), nextAttr.voiceColor ?? GetParentVoiceColor(), out var oto)) {

                    if (oto.Overlap < 0) {
                        vcLength = MsToTick(oto.Preutter - oto.Overlap);
                    } else {
                        vcLength = MsToTick(oto.Preutter);
                    }
                }

                vcLength = Convert.ToInt32(Math.Min(totalDuration / 2, vcLength * (nextAttr.consonantStretchRatio ?? GetParentConsonantStretchRatio())));

                return new Result {
                    phonemes = new Phoneme[] {
                        new Phoneme() {
                            phoneme = currentLyric,
                        },
                        new Phoneme() {
                            phoneme = vcPhoneme,
                            position = totalDuration - vcLength,
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
    }
}
