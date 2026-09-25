using System;
using System.Collections.Generic;
using System.Text;
using OpenUtau.Api;
using System.Linq;

namespace OpenUtau.Plugin.Builtin {
    [Phonemizer("French CVVC Phonemizer", "FR CVVC", "Mim", language: "FR")]

    public class FrenchCVVCPhonemizer : SyllableBasedPhonemizer {

        private readonly string[] vowels = "ah,ae,eh,ee,oe,ih,oh,oo,ou,uh,en,in,on,oi,ui,a,ai,e,i,o,u,eu".Split(",");
        private readonly string[] consonants = "b,d,f,g,j,k,l,m,n,p,r,s,sh,t,v,w,y,z,gn,.,-,R,BR,_hh".Split(",");
        private readonly Dictionary<string, string> dictionaryReplacements = (
            "aa=ah;ai=ae;ei=eh;eu=ee;ee=ee;oe=oe;ii=ih;au=oh;oo=oo;ou=ou;uu=uh;an=en;in=in;un=in;on=on;uy=ui;" +
            "bb=b;dd=d;ff=f;gg=g;jj=j;kk=k;ll=l;mm=m;nn=n;pp=p;rr=r;ss=s;ch=sh;tt=t;vv=v;ww=w;yy=y;zz=z;gn=gn;4=l;hh=h;").Split(';')
                .Select(entry => entry.Split('='))
                .Where(parts => parts.Length == 2)
                .Where(parts => parts[0] != parts[1])
                .ToDictionary(parts => parts[0], parts => parts[1]);

        private string[] shortConsonants = "r".Split(",");
        private string[] longConsonants = "t,k,g,p,s,sh,j".Split(",");

        private readonly Dictionary<string, string> fraloidsReplacement = (
            "ah=a;ae=ai;ee=e;ih=i;oh=o;uh=u;oe=eu;").Split(';')
                .Select(entry => entry.Split('='))
                .Where(parts => parts.Length == 2)
                .Where(parts => parts[0] != parts[1])
                .ToDictionary(parts => parts[0], parts => parts[1]);

        private bool usesFraloids = false;
        protected override string[] GetVowels() => vowels;
        protected override string[] GetConsonants() => consonants;
        protected override string GetDictionaryName() => "cmudict_fr.txt";
        protected override Dictionary<string, string> GetDictionaryPhonemesReplacement() => dictionaryReplacements;

        protected override List<string> ProcessSyllable(Syllable syllable) {
            string prevV = syllable.prevV;
            string[] cc = syllable.cc;
            string v = syllable.v;
            var lastC = cc.Length - 1;
            var firstC = 0;

            string basePhoneme;
            var phonemes = new List<string>();

            if (prevV == "ui") {
                prevV = "ih";
            }

            if (HasOto("a", syllable.tone)) {
                usesFraloids = true;
                v = ValidateAlias(v);
                prevV = ValidateAlias(prevV);
            }

            if (syllable.IsStartingV) {

                basePhoneme = CheckAliasFormatting(v, "cv", syllable.vowelTone, "");

            } else if (syllable.IsVV) {
                if (!CanMakeAliasExtension(syllable)) {
                    var vvCheck = prevV + v;

                    if (usesFraloids && vvCheck == "ai") {
                        basePhoneme = CheckAliasFormatting(v, "vvFr", syllable.vowelTone, prevV);
                    } else {
                        basePhoneme = CheckAliasFormatting(v, "vv", syllable.vowelTone, prevV);
                        if (basePhoneme == v) {

                            if (prevV == "ih" || prevV == "i") {
                                basePhoneme = $"y{v}";
                            }
                            if (prevV == "ou") {
                                basePhoneme = $"w{v}";
                            }
                            if (!HasOto(basePhoneme, syllable.tone))
                                basePhoneme = v;
                        }
                    }

                } else {

                    basePhoneme = null;
                }

            } else if (syllable.IsStartingCVWithOneConsonant) {

                var cv = $"{cc[0]}{v}";
                basePhoneme = CheckAliasFormatting(cv, "cv", syllable.tone, "");

                if (!basePhoneme.Contains("-")) {
                    TryAddPhoneme(phonemes, syllable.tone, CheckAliasFormatting($"{cc[0]}", "rcv", syllable.tone, ""));
                }

            } else if (syllable.IsStartingCVWithMoreThanOneConsonant) {

                var rccv = $"{string.Join("", cc)}{v}";

                rccv = CheckAliasFormatting(rccv, "rcv", syllable.tone, "");
                if (HasOto(rccv, syllable.vowelTone)) {
                    basePhoneme = rccv;
                } else {
                    basePhoneme = $"{cc.Last()}{v}";

                    int max = cc.Length;
                    int min = 0;

                    for (int i = 0; i < cc.Length; i++) {
                        string rcc = "";

                        for (int k = 0; k < max; k++) {
                            rcc += $"{cc[k]}";
                        }
                        rcc = CheckAliasFormatting(rcc, "rcv", syllable.tone, "");
                        if (HasOto(rcc, syllable.tone)) {
                            phonemes.Add(rcc);
                            break;
                        }
                        max--;
                    }

                    if (FindLastValidAlias(phonemes, cc) == cc.Length - 1) {

                    } else {
                        min = cc.Length - FindLastValidAlias(phonemes, cc);
                        max = cc.Length;

                        for (var i = 0; i < min; i++) {

                            rccv = "";
                            for (var k = i; k < cc.Length; k++) {
                                rccv += $"{cc[k]}";
                            }
                            rccv += $"{v}";

                            if (!HasOto(rccv, syllable.tone)) {
                                rccv = ValidateAlias(rccv);
                            }

                            if (HasOto(rccv, syllable.vowelTone)) {
                                basePhoneme = rccv;
                                break;
                            }
                            max--;
                        }

                        if (HasOto($"_{cc.Last()}{v}", syllable.vowelTone) && max == cc.Length - min) {
                            basePhoneme = $"_{cc.Last()}{v}";
                        }

                        for (int i = 0; i < cc.Length - max; i++) {
                            string rcc = "";

                            for (int k = 0; k < min; k++) {
                                rcc += $"{cc[k]}";
                            }

                            if (HasOto(rcc, syllable.tone)) {
                                phonemes.Add(rcc);
                                break;
                            }
                            min--;
                        }

                        if (FindLastValidAlias(phonemes, cc) == cc.Length - 1) {

                        } else {

                            min = FindLastValidAlias(phonemes, cc);

                            for (int i = min; i < cc.Length - max; i++) {
                                var ccc = $"{cc[i]}";
                                ccc = CheckAliasFormatting(ccc, "endccOe", syllable.tone, $"{cc[i]}");

                                if ($"{cc[i + 1]}" == "y" && ccc.Contains(CheckCoeEnding(ccc, syllable.tone))) {
                                    if (usesFraloids)
                                        ccc = $"{cc[i]}i";
                                    else
                                        ccc = $"{cc[i]}ih";
                                }
                                phonemes.Add(ccc);

                            }
                        }

                    }
                }
            }

                else if (syllable.IsVCVWithOneConsonant) {

                var vcv = $"{prevV} {cc[0]}{v}";
                if (HasOto(vcv, syllable.vowelTone)) {
                    basePhoneme = vcv;
                } else {
                    var cv = $"{cc[0]}{v}";

                    basePhoneme = cv;

                    var vc = "";

                    if (usesFraloids) {
                        vc = $"{prevV} {cc[0]}";
                        vc = ReplaceFraloidsConflict(vc, syllable.tone);
                        if (HasOto(vc, syllable.tone)) {
                            phonemes.Add(vc);
                        } else {
                            vc = $"{prevV}{cc[0]}";
                            if (HasOto(vc, syllable.tone)) {
                                phonemes.Add(vc);
                            }
                        }
                    }
                    if (phonemes.Count == 0) {
                        vc = CheckAliasFormatting(cc[0], "vc", syllable.tone, prevV);
                        if (HasOto(vc, syllable.tone)) {
                            phonemes.Add(vc);
                        }
                    }

                }
            } else {

                basePhoneme = $"{cc.Last()}{v}";

                var max = cc.Length;
                var min = 0;

                var vc = "";

                if (usesFraloids) {
                    vc = $"{prevV} {cc[0]}";
                    vc = ReplaceFraloidsConflict(vc, syllable.tone);
                    if (HasOto(vc, syllable.tone)) {
                        phonemes.Add(vc);
                    }
                }

                if (phonemes.Count == 0) {

                    var vcc = "";
                    for (var i = 0; i < cc.Length; i++) {

                        vcc = "";
                        for (var k = 0; k < cc.Length - i; k++) {
                            vcc += $"{cc[k]}";
                        }

                        vcc = CheckAliasFormatting(vcc, "vc", syllable.tone, prevV);

                        if (HasOto(vcc, syllable.tone)) {
                            phonemes.Add(vcc);
                            break;
                        }
                    }
                }

                min = cc.Length - FindLastValidAlias(phonemes, cc);
                max = cc.Length;

                var ccv = "";
                for (var i = 0; i < min; i++) {

                    ccv = "";
                    for (var k = i; k < cc.Length; k++) {
                        ccv += $"{cc[k]}";
                    }
                    ccv += $"{v}";

                    if (!HasOto(ccv, syllable.tone)) {
                        ccv = ValidateAlias(ccv);
                    }

                    if (HasOto(ccv, syllable.vowelTone)) {
                        basePhoneme = ccv;
                        break;
                    }
                    max--;
                }

                if (HasOto($"_{cc.Last()}{v}", syllable.vowelTone) && max == cc.Length - min) {
                    basePhoneme = $"_{cc.Last()}{v}";
                }

                min = FindLastValidAlias(phonemes, cc);

                if (min == cc.Length) {

                } else {
                    min--;

                    for (int i = min; i < cc.Length - max; i++) {
                        var ccc = $"{cc[i]}";

                        if (i + 1 >= cc.Length) {
                            break;
                        }

                        ccc = CheckAliasFormatting(ccc, "endccOe", syllable.tone, $"{cc[i + 1]}");

                        if (ccc.Contains(CheckCoeEnding(ccc, syllable.tone)) || ccc == $"{cc[i]}") {

                            if (cc[i] == cc[i + 1]) {
                                break;
                            }
                            if (i == 0 && $"{cc[i + 1]}" != "y") {
                                continue;
                            }
                        }

                        if ($"{cc[i + 1]}" == "y" && ccc.Contains(CheckCoeEnding(ccc, syllable.tone))) {
                            if (usesFraloids)
                                ccc = $"{cc[i]}i";
                            else
                                ccc = $"{cc[i]}ih";
                        }

                        if (ccc == $"{cc[i]}") {
                            if (i + 2 <= cc.Length) {
                                break;
                            }
                        }

                        phonemes.Add(ccc);

                    }
                }

            }

            phonemes.Add(basePhoneme);
            return phonemes;
        }
        protected override List<string> ProcessEnding(Ending ending) {
            string[] cc = ending.cc;
            string v = ending.prevV;

            var phonemes = new List<string>();
            bool hasEnding = false;

            if (HasOto("a", ending.tone)) {
                v = ValidateAlias(v);
            }

            if (ending.IsEndingV) {

                var endV = CheckAliasFormatting(v, "end", ending.tone, "");
                TryAddPhoneme(phonemes, ending.tone, endV);

                if (phonemes.Count == 0) {
                    endV = v + " R";
                    TryAddPhoneme(phonemes, ending.tone, endV);
                    if (phonemes.Count == 0) {
                        endV = v + " BR";
                        TryAddPhoneme(phonemes, ending.tone, endV);
                        if (phonemes.Count == 0) {
                            endV = v + "_hh";
                            TryAddPhoneme(phonemes, ending.tone, endV);
                        }
                    }
                }

            } else {

                if (ending.IsEndingVCWithOneConsonant) {

                    var vc = "";

                    if (usesFraloids) {
                        vc = $"{v} {cc[0]}";
                        vc = ReplaceFraloidsConflict(vc, ending.tone);
                        if (HasOto(vc, ending.tone)) {
                            phonemes.Add(vc);
                        }
                    }
                    if (phonemes.Count == 0) {

                        vc = CheckAliasFormatting($"{v}{cc[0]}", "endVc", ending.tone, "");
                        if (HasOto(vc, ending.tone)) {
                            phonemes.Add(vc);
                        } else {
                            vc = CheckAliasFormatting(cc[0], "vc", ending.tone, v);
                            phonemes.Add(vc);
                        }
                    }

                    if (!vc.Contains("-")) {
                        TryAddPhoneme(phonemes, ending.tone, CheckAliasFormatting($"{cc[0]}", "end", ending.tone, ""));
                    }

                } else {

                    var max = cc.Length;

                    var vc = "";

                    if (usesFraloids) {
                        vc = $"{v} {cc[0]}";
                        vc = ReplaceFraloidsConflict(vc, ending.tone);
                        if (HasOto(vc, ending.tone)) {
                            phonemes.Add(vc);
                        }
                    }
                    if (phonemes.Count == 0) {

                        var vcc = "";
                        for (var i = 0; i < cc.Length; i++) {

                            string type = "endVc";
                            if (i > 0) { type = "blank"; }

                            vcc = "";
                            for (var k = 0; k < cc.Length - i; k++) {
                                vcc += $"{cc[k]}";
                            }
                            var temp = $"{v}{vcc}";
                            temp = CheckAliasFormatting(temp, type, ending.tone, "");

                            if (HasOto(temp, ending.tone)) {
                                vcc = temp;
                                phonemes.Add(vcc);
                                break;
                            } else {
                                temp = $"{v} {vcc}";
                                temp = CheckAliasFormatting(temp, type, ending.tone, "");
                                if (HasOto(temp, ending.tone)) {
                                    vcc = temp;
                                    phonemes.Add(vcc);
                                    break;
                                }
                            }
                            max--;
                        }
                    }

                    if (FindLastValidAlias(phonemes, cc) == cc.Length) {

                        var end = CheckAliasFormatting($"{cc.Last()}", "end", ending.tone, "");
                        TryAddPhoneme(phonemes, ending.tone, end);

                    } else {

                        for (int i = max - 1; i < cc.Length; i++) {

                            var ccc = $"{cc[i]}";

                            if (i + 1 == cc.Length - 1) {
                                ccc = $"{cc[i]}{cc[i + 1]}";
                                ccc = CheckAliasFormatting(ccc, "end", ending.tone, "");
                                if (HasOto(ccc, ending.tone)) {
                                    phonemes.Add(ccc);
                                    break;
                                }
                            }

                            if (i + 1 < cc.Length) {

                                ccc = $"{cc[i]}";
                                ccc = CheckAliasFormatting(ccc, "endcc", ending.tone, $"{cc[i + 1]}");
                                if (HasOto(ccc, ending.tone)) {
                                    phonemes.Add(ccc);
                                    continue;
                                }
                            }

                            if (i > 0) {
                                ccc = $"{cc[i]}";
                                ccc = CheckAliasFormatting(ccc, "endcOe", ending.tone, "");
                            }

                            if (HasOto(ccc, ending.tone)) {
                                phonemes.Add(ccc);
                            }

                        }

                    }
                }

            }

            return phonemes;
        }

        protected override string ValidateAlias(string alias, int tone = 0) {
            if (HasOto(alias, tone)) return alias;

            string baseResolved = base.ValidateAlias(alias, tone);
            if (!string.IsNullOrEmpty(baseResolved) && baseResolved != alias) {
                if (HasOto(baseResolved, tone)) {
                    return baseResolved;
                }
                alias = baseResolved;
            }

            if (usesFraloids) {
                foreach (var syllable in fraloidsReplacement) {
                    alias = alias.Replace(syllable.Key, syllable.Value);
                }
            }

            foreach (var oi in new[] { "wah", "wa" }) {
                alias = alias.Replace(oi, "oi");
            }

            return alias;
        }

        private string ReplaceFraloidsConflict(string vc, int tone) {
            Dictionary<string, string> fraloidsVCs = new Dictionary<string, string> {
                {"o n","on2"},
                {"e n","en2"},
                {"i n","in2"},
                {"u n","un2"},
            };

            if (HasOto(vc, tone) || vc == "ai n") {
                return vc;
            }

            foreach (var vcFr in fraloidsVCs) {
                vc = vc.Replace(vcFr.Key, vcFr.Value);
            }

            return vc;
        }

        private string CheckCoeEnding(string cv, int tone) {

            if (HasOto(cv, tone) && cv.Contains("oe")) {
                cv = "oe";
                return cv;
            } else if (cv.Contains("eu")) {
                cv = "eu";
                return cv;
            }

            return "no Coe Ending";
        }

        protected override bool NoGap => true;

        protected override double GetTransitionBasicLengthMs(string alias, int tone, PhonemeAttributes attr) {
            double otoLength = GetTransitionBasicLengthMsByOto(alias, tone, attr);
            return otoLength;
        }

        private string CheckAliasFormatting(string alias, string type, int tone, string prevV) {

            var aliasFormat = "";
            string[] aliasFormats = new string[] { "-", "- ", "", "-", " -", "", prevV, prevV + " ", "_", "", prevV, " " + prevV, "", "oe", "eu" };
            var startingI = 0;
            var endingI = aliasFormats.Length;

            if (type == "end") {
                startingI = 3;
                endingI = startingI + 1;
            }

            if (type == "endC") {
                startingI = 2;
                endingI = startingI + 1;
            }

            if (type == "endVc") {
                startingI = 3;
                endingI = startingI + 2;
            }

            if (type == "vc") {
                startingI = 6;
                endingI = startingI + 1;
            }

            if (type == "endcc") {
                startingI = 6;
                endingI = startingI + 1;
            }

            if (type == "rcv") {
                startingI = 0;
                endingI = startingI + 1;
            }

            if (type == "vv") {
                startingI = 6;
                endingI = startingI + 3;
            }
            if (type == "vvFr") {
                startingI = 7;
                endingI = startingI + 3;
            }

            if (type == "cc") {
                startingI = 6;
                endingI = startingI + 1;
            }

            if (type == "endccOe") {
                startingI = 10;
                endingI = startingI + 4;
            }

            if (type == "endcOe") {
                startingI = 12;
                endingI = startingI + 2;
            }

            if (type == "blank") {
                startingI = 2;
                endingI = startingI;
            }

            if (type == "cv") {
                startingI = 0;
                endingI = startingI + 2;
            }

            for (int i = startingI; i <= endingI; i++) {
                if (type.Contains("end")) {
                    aliasFormat = alias + aliasFormats[i];
                } else aliasFormat = aliasFormats[i] + alias;

                if (HasOto(aliasFormat, tone)) {
                    alias = aliasFormat;
                    return alias;
                }
            }

            return "no alias found";
        }

        private int FindLastValidAlias(List<string> inputPhonemes, string[] wordPhonemes) {
            var lastAliasIndex = 0;
            for (int i = 0; i < inputPhonemes.Count; i++) {
                lastAliasIndex = 0;
                for (int k = 0; k < wordPhonemes.Length; k++) {
                    if (!inputPhonemes[i].Contains($"{wordPhonemes[k]}")) {
                        break;
                    }
                    lastAliasIndex++;
                }
            }

            return lastAliasIndex;
        }

        protected override string[] GetSymbols(Note note) {
            string[] original = base.GetSymbols(note);
            if (original == null) {
                return null;
            }

            string[] arpabet = "aa,ai,ei,eu,ii,au,uu,an,un,uy,bb,dd,ff,gg,jj,kk,ll,mm,nn,pp,rr,ss,ch,tt,vv,ww,yy,zz".Split(",");
            string[] petitmot = "ah,ae,eh,ee,ih,oh,uh,en,in,ui,b,d,f,g,j,k,l,m,n,p,r,s,sh,t,v,w,y,z".Split(",");

            List<string> convert = new List<string>();
            foreach (string s in original) {
                string c = s;
                for (int i = 0; i < arpabet.Length; i++) {
                    if (s == arpabet[i]) {
                        c = petitmot[i];
                    }
                }
                convert.Add(c);
            }

            if (convert == null) {
                return null;
            }

            List<string> modified = new List<string>();
            foreach (string s in convert) {
                if (s == "gn") {
                    modified.AddRange(new string[] { "n", "y" });
                } else {
                    modified.Add(s);
                }
            }
            return modified.ToArray();
        }

    }
}
