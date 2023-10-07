#![allow(unused)]

pub mod convert;
pub mod split;

pub mod traits {
    use rx::prelude::*;

    pub trait PcmReader {
        fn props(&mut self) -> AnyResult<FileProps>;

        fn read(
            &mut self,
            buf: &mut Buf,
        ) -> AnyResult<()>;
    }

    pub trait PcmWriter {
        fn props(&self) -> AnyResult<FileProps>;

        fn write(
            &mut self,
            buf: &Buf,
        ) -> AnyResult<()>;
    }

    pub enum Buf {
        Uninit,
        F32(Vec<f32>),
        I24(Vec<i32>),
        I16(Vec<i16>),
    }

    pub struct FileProps {
        pub format: Format,
        pub bit_depth: BitDepth,
        pub sample_rate: SampleRate,
    }

    pub enum Format {
        Wav,
        Flac,
        Vorbis,
    }

    pub enum BitDepth {
        F32,
        I24,
        I16,
    }

    pub enum SampleRate {
        K192,
        K48,
    }

    fn static_assertions(
        reader: &dyn PcmReader,
        writer: &dyn PcmWriter,
    ) { }
}

pub mod codecs {
    pub mod wav {
    }

    pub mod flac {
    }

    pub mod vorbis {
    }
}

pub mod samplerate {
}

pub mod bitdepth {
    pub fn f32_to_i24(
        inbuf: &[f32],
        outbuf: &mut [i32],
    ) {
        todo!()
    }

    pub fn f32_to_i16(
        inbuf: &[f32],
        outbuf: &mut [i16],
    ) {
        todo!()
    }

    pub fn i24_to_i16(
        inbuf: &[i32],
        outbuf: &mut [i16],
    ) {
        todo!()
    }
}

pub mod dither {
    pub fn i24(
        inbuf: &[i32],
        outbuf: &mut [i32],
    ) {
        todo!()
    }
}
