pub fn generate_openutau_api_contract() -> &'static str {
    r#"// ==========================================================================
// OpenUtau API Interfaces & Contract Definitions
// ==========================================================================
using System;
using System.Collections.Generic;

namespace OpenUtau.Api
{
    [AttributeUsage(AttributeTargets.Class, Inherited = false, AllowMultiple = false)]
    public sealed class PhonemizerAttribute : Attribute
    {
        public string Name { get; }
        public string Tag { get; }
        public string Author { get; }
        public string Language { get; }

        public PhonemizerAttribute(string name, string tag, string author, string language = "")
        {
            Name = name;
            Tag = tag;
            Author = author;
            Language = language;
        }
    }

    public struct Note
    {
        public string lyric;
        public int position;
        public int duration;
        public int tone;
        public string toneName => "C4";
        public string phoneticHint;
    }

    public struct Phoneme
    {
        public string phoneme;
        public int position;
    }

    public struct Result
    {
        public Phoneme[] phonemes;
    }

    public abstract class Phonemizer
    {
        public virtual void SetSinger(Core.Ustx.USinger singer) { }
        public virtual void SetTiming(int[] timings) { }
        public abstract Result Process(Note[] notes, Note? prev, Note? next, Note? prevNeighbour, Note? nextNeighbour, Note[] prevs);
        public virtual void CleanUp() { }
    }

    public abstract class SyllableBasedPhonemizer : Phonemizer
    {
        // OpenUtau Syllable base helper abstraction
    }
}

namespace OpenUtau.Core.Ustx
{
    public class UOto
    {
        public string Alias { get; set; } = string.Empty;
        public string File { get; set; } = string.Empty;
        public double Offset { get; set; }
        public double Consonant { get; set; }
        public double Cutoff { get; set; }
        public double Preutter { get; set; }
        public double Overlap { get; set; }
    }

    public class USinger
    {
        public string Id { get; set; } = string.Empty;
        public string Name { get; set; } = string.Empty;
        public virtual bool TryGetOto(string phoneme, out UOto? oto)
        {
            oto = new UOto { Alias = phoneme };
            return true;
        }
    }
}
"#
}
