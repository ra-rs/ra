#[doc = "Register `CONTROL2` writer"]
pub struct W(crate::W<CONTROL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CONTROL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pixel source is a pattern color (blend of COLOR1 and COLOR2 depending on PATTERN and pattern index)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PATTERNENABLE_AW {
    #[doc = "0: disabled pattern"]
    _0 = 0,
    #[doc = "1: enabled pattern"]
    _1 = 1,
}
impl From<PATTERNENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: PATTERNENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PATTERNENABLE` writer - Pixel source is a pattern color (blend of COLOR1 and COLOR2 depending on PATTERN and pattern index)"]
pub type PATTERNENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL2_SPEC, PATTERNENABLE_AW, O>;
impl<'a, const O: u8> PATTERNENABLE_W<'a, O> {
    #[doc = "disabled pattern"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PATTERNENABLE_AW::_0)
    }
    #[doc = "enabled pattern"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PATTERNENABLE_AW::_1)
    }
}
#[doc = "Pixel source is read from texture and used as an alpha to blend between COLOR1 and COLOR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTUREENABLE_AW {
    #[doc = "0: disabled texture"]
    _0 = 0,
    #[doc = "1: enabled texture"]
    _1 = 1,
}
impl From<TEXTUREENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: TEXTUREENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXTUREENABLE` writer - Pixel source is read from texture and used as an alpha to blend between COLOR1 and COLOR2"]
pub type TEXTUREENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL2_SPEC, TEXTUREENABLE_AW, O>;
impl<'a, const O: u8> TEXTUREENABLE_W<'a, O> {
    #[doc = "disabled texture"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEXTUREENABLE_AW::_0)
    }
    #[doc = "enabled texture"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEXTUREENABLE_AW::_1)
    }
}
#[doc = "Field `PATTERNSOURCEL5` writer - Limiter 5 is used as pattern index instead of the default U limiter.Limiter 5 can be combined with limiter 6 to form a quadratic limiter which can be used to make quadratic pattern functions to draw radial patterns."]
pub type PATTERNSOURCEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, bool, O>;
#[doc = "Alpha blend mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USEACB_AW {
    #[doc = "0: use WRITEALPHA\\[1:0\\]
mode"]
    _0 = 0,
    #[doc = "1: use full alpha channel blending mode"]
    _1 = 1,
}
impl From<USEACB_AW> for bool {
    #[inline(always)]
    fn from(variant: USEACB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEACB` writer - Alpha blend mode"]
pub type USEACB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, USEACB_AW, O>;
impl<'a, const O: u8> USEACB_W<'a, O> {
    #[doc = "use WRITEALPHA\\[1:0\\]
mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USEACB_AW::_0)
    }
    #[doc = "use full alpha channel blending mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USEACB_AW::_1)
    }
}
#[doc = "Field `READFORMAT32` writer - Bit 4 and 3 of the texture buffer format.See READFORMAT above for description"]
pub type READFORMAT32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL2_SPEC, u8, u8, 2, O>;
#[doc = "Blend source factor for alpha channel in alpha channel blending mode (USEACB = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSFA_AW {
    #[doc = "0: use 1.0 as blend source factor for alpha channel"]
    _0 = 0,
    #[doc = "1: use alpha as blend source factor for alpha channel"]
    _1 = 1,
}
impl From<BSFA_AW> for bool {
    #[inline(always)]
    fn from(variant: BSFA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSFA` writer - Blend source factor for alpha channel in alpha channel blending mode (USEACB = 1)"]
pub type BSFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, BSFA_AW, O>;
impl<'a, const O: u8> BSFA_W<'a, O> {
    #[doc = "use 1.0 as blend source factor for alpha channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSFA_AW::_0)
    }
    #[doc = "use alpha as blend source factor for alpha channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSFA_AW::_1)
    }
}
#[doc = "Blend destinetion factor for alpha channel in alpha channel blending mode (USEACB = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDFA_AW {
    #[doc = "0: use 1.0 as blend destination factor for alpha channel"]
    _0 = 0,
    #[doc = "1: use alpha as blend destination factor for alpha channel"]
    _1 = 1,
}
impl From<BDFA_AW> for bool {
    #[inline(always)]
    fn from(variant: BDFA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDFA` writer - Blend destinetion factor for alpha channel in alpha channel blending mode (USEACB = 1)"]
pub type BDFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, BDFA_AW, O>;
impl<'a, const O: u8> BDFA_W<'a, O> {
    #[doc = "use 1.0 as blend destination factor for alpha channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BDFA_AW::_0)
    }
    #[doc = "use alpha as blend destination factor for alpha channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BDFA_AW::_1)
    }
}
#[doc = "Field `WRITEFORMAT2` writer - Bit 3 of framebuffer pixel formatSee WRITEFORMAT above description."]
pub type WRITEFORMAT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, bool, O>;
#[doc = "Blend source factorsrc factor is alpha (factor is 1 per default)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSF_AW {
    #[doc = "0: use 1.0 as blend source factor"]
    _0 = 0,
    #[doc = "1: use alpha as blend source factor"]
    _1 = 1,
}
impl From<BSF_AW> for bool {
    #[inline(always)]
    fn from(variant: BSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSF` writer - Blend source factorsrc factor is alpha (factor is 1 per default)"]
pub type BSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, BSF_AW, O>;
impl<'a, const O: u8> BSF_W<'a, O> {
    #[doc = "use 1.0 as blend source factor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSF_AW::_0)
    }
    #[doc = "use alpha as blend source factor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSF_AW::_1)
    }
}
#[doc = "Blend destination factordst factor is alpha (factor is 1 per default)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDF_AW {
    #[doc = "0: use 1.0 as blend destination factor"]
    _0 = 0,
    #[doc = "1: use alpha as blend destination factor"]
    _1 = 1,
}
impl From<BDF_AW> for bool {
    #[inline(always)]
    fn from(variant: BDF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDF` writer - Blend destination factordst factor is alpha (factor is 1 per default)"]
pub type BDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, BDF_AW, O>;
impl<'a, const O: u8> BDF_W<'a, O> {
    #[doc = "use 1.0 as blend destination factor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BDF_AW::_0)
    }
    #[doc = "use alpha as blend destination factor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BDF_AW::_1)
    }
}
#[doc = "Blend source factor is invertedsrc factor will be inverted (meaning 1-a or 1-1 depending on BSF)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSI_AW {
    #[doc = "0: use blend factor as specified through BSF"]
    _0 = 0,
    #[doc = "1: invert blend source factor (1-x)"]
    _1 = 1,
}
impl From<BSI_AW> for bool {
    #[inline(always)]
    fn from(variant: BSI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSI` writer - Blend source factor is invertedsrc factor will be inverted (meaning 1-a or 1-1 depending on BSF)"]
pub type BSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, BSI_AW, O>;
impl<'a, const O: u8> BSI_W<'a, O> {
    #[doc = "use blend factor as specified through BSF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSI_AW::_0)
    }
    #[doc = "invert blend source factor (1-x)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSI_AW::_1)
    }
}
#[doc = "Blend destination factor is inverteddst factor will be inverted (meaning 1-a or 1-1 depending on BDF)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDI_AW {
    #[doc = "0: use blend factor as specified through BDF"]
    _0 = 0,
    #[doc = "1: invert blend destinationfactor (1-x)"]
    _1 = 1,
}
impl From<BDI_AW> for bool {
    #[inline(always)]
    fn from(variant: BDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDI` writer - Blend destination factor is inverteddst factor will be inverted (meaning 1-a or 1-1 depending on BDF)"]
pub type BDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, BDI_AW, O>;
impl<'a, const O: u8> BDI_W<'a, O> {
    #[doc = "use blend factor as specified through BDF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BDI_AW::_0)
    }
    #[doc = "invert blend destinationfactor (1-x)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BDI_AW::_1)
    }
}
#[doc = "Blend color 2 instead of framebuffer pixel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BC2_AW {
    #[doc = "0: use pixel from framebuffer as destination (DST)"]
    _0 = 0,
    #[doc = "1: use color 2 as destination (DST)"]
    _1 = 1,
}
impl From<BC2_AW> for bool {
    #[inline(always)]
    fn from(variant: BC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC2` writer - Blend color 2 instead of framebuffer pixel"]
pub type BC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, BC2_AW, O>;
impl<'a, const O: u8> BC2_W<'a, O> {
    #[doc = "use pixel from framebuffer as destination (DST)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BC2_AW::_0)
    }
    #[doc = "use color 2 as destination (DST)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BC2_AW::_1)
    }
}
#[doc = "Calculating U limiter outside use textureThe bit describes what happens if the U limiter (x direction in texture space) calculates a U value outside of the used texture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTURECLAMPX_AW {
    #[doc = "0: Texture wrap mode: The integer part of the calculated value from the u limiter is anded with TEXUMASK. This results in a repetition of the texture in x/u direction."]
    _0 = 0,
    #[doc = "1: Texture clamp mode: The texture color at the border of the texture is taken. This results in a repetition of the texture border color in x/u direction."]
    _1 = 1,
}
impl From<TEXTURECLAMPX_AW> for bool {
    #[inline(always)]
    fn from(variant: TEXTURECLAMPX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXTURECLAMPX` writer - Calculating U limiter outside use textureThe bit describes what happens if the U limiter (x direction in texture space) calculates a U value outside of the used texture"]
pub type TEXTURECLAMPX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL2_SPEC, TEXTURECLAMPX_AW, O>;
impl<'a, const O: u8> TEXTURECLAMPX_W<'a, O> {
    #[doc = "Texture wrap mode: The integer part of the calculated value from the u limiter is anded with TEXUMASK. This results in a repetition of the texture in x/u direction."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEXTURECLAMPX_AW::_0)
    }
    #[doc = "Texture clamp mode: The texture color at the border of the texture is taken. This results in a repetition of the texture border color in x/u direction."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEXTURECLAMPX_AW::_1)
    }
}
#[doc = "Calculating V limiter outside use textureThe bit describes what happens if the V limiter (y direction in texture space) calculates a V value outside of the used texture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTURECLAMPY_AW {
    #[doc = "0: Texture wrap mode: The integer part of the calculated value from the v limiter is anded with TEXVMASK. This results in a repetition of the texture in y/v direction."]
    _0 = 0,
    #[doc = "1: Texture clamp mode: The texture color at the border of the texture is taken. This results in a repetition of the texture border color in y/v direction."]
    _1 = 1,
}
impl From<TEXTURECLAMPY_AW> for bool {
    #[inline(always)]
    fn from(variant: TEXTURECLAMPY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXTURECLAMPY` writer - Calculating V limiter outside use textureThe bit describes what happens if the V limiter (y direction in texture space) calculates a V value outside of the used texture"]
pub type TEXTURECLAMPY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL2_SPEC, TEXTURECLAMPY_AW, O>;
impl<'a, const O: u8> TEXTURECLAMPY_W<'a, O> {
    #[doc = "Texture wrap mode: The integer part of the calculated value from the v limiter is anded with TEXVMASK. This results in a repetition of the texture in y/v direction."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEXTURECLAMPY_AW::_0)
    }
    #[doc = "Texture clamp mode: The texture color at the border of the texture is taken. This results in a repetition of the texture border color in y/v direction."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEXTURECLAMPY_AW::_1)
    }
}
#[doc = "Linear filtering on texture U axis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTUREFILTERX_AW {
    #[doc = "0: no filtering on texture U axis"]
    _0 = 0,
    #[doc = "1: linear filtering on texture U axis"]
    _1 = 1,
}
impl From<TEXTUREFILTERX_AW> for bool {
    #[inline(always)]
    fn from(variant: TEXTUREFILTERX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXTUREFILTERX` writer - Linear filtering on texture U axis"]
pub type TEXTUREFILTERX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL2_SPEC, TEXTUREFILTERX_AW, O>;
impl<'a, const O: u8> TEXTUREFILTERX_W<'a, O> {
    #[doc = "no filtering on texture U axis"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEXTUREFILTERX_AW::_0)
    }
    #[doc = "linear filtering on texture U axis"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEXTUREFILTERX_AW::_1)
    }
}
#[doc = "Linear filtering on texture V axis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTUREFILTERY_AW {
    #[doc = "0: no filtering on texture V axis"]
    _0 = 0,
    #[doc = "1: linear filtering on texture V axis"]
    _1 = 1,
}
impl From<TEXTUREFILTERY_AW> for bool {
    #[inline(always)]
    fn from(variant: TEXTUREFILTERY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXTUREFILTERY` writer - Linear filtering on texture V axis"]
pub type TEXTUREFILTERY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL2_SPEC, TEXTUREFILTERY_AW, O>;
impl<'a, const O: u8> TEXTUREFILTERY_W<'a, O> {
    #[doc = "no filtering on texture V axis"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEXTUREFILTERY_AW::_0)
    }
    #[doc = "linear filtering on texture V axis"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEXTUREFILTERY_AW::_1)
    }
}
#[doc = "Pixel format of the texture buffer{READFORMAT32,READFORMAT10}0000: 8 bpp a(8)0001: 16 bpp RGB(565)0010: 32 bpp aRGB(8888)0011: 16 bpp aRGB(4444)0100: 16 bpp aRGB(1555)0101: 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color1001: 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance1010: 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance1011: 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance 1100: 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum READFORMAT10_AW {
    #[doc = "0: 8 bpp a(8) (READFORMAT32=00) / 16 bpp aRGB(1555) (READFORMAT32=01) / 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance (READFORMAT32=11)"]
    _00 = 0,
    #[doc = "1: 16 bpp RGB(565) (READFORMAT32=00) / 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color (READFORMAT32=01) / 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance (READFORMAT32=10)"]
    _01 = 1,
    #[doc = "2: 32 bpp aRGB(8888) (READFORMAT32=00) / 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance (READFORMAT32=10)"]
    _10 = 2,
    #[doc = "3: 16 bpp aRGB(4444) (READFORMAT32=00) / 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance (READFORMAT32=10)"]
    _11 = 3,
}
impl From<READFORMAT10_AW> for u8 {
    #[inline(always)]
    fn from(variant: READFORMAT10_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `READFORMAT10` writer - Pixel format of the texture buffer{READFORMAT32,READFORMAT10}0000: 8 bpp a(8)0001: 16 bpp RGB(565)0010: 32 bpp aRGB(8888)0011: 16 bpp aRGB(4444)0100: 16 bpp aRGB(1555)0101: 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color1001: 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance1010: 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance1011: 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance 1100: 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance"]
pub type READFORMAT10_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONTROL2_SPEC, u8, READFORMAT10_AW, 2, O>;
impl<'a, const O: u8> READFORMAT10_W<'a, O> {
    #[doc = "8 bpp a(8) (READFORMAT32=00) / 16 bpp aRGB(1555) (READFORMAT32=01) / 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance (READFORMAT32=11)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(READFORMAT10_AW::_00)
    }
    #[doc = "16 bpp RGB(565) (READFORMAT32=00) / 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color (READFORMAT32=01) / 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance (READFORMAT32=10)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(READFORMAT10_AW::_01)
    }
    #[doc = "32 bpp aRGB(8888) (READFORMAT32=00) / 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance (READFORMAT32=10)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(READFORMAT10_AW::_10)
    }
    #[doc = "16 bpp aRGB(4444) (READFORMAT32=00) / 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance (READFORMAT32=10)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(READFORMAT10_AW::_11)
    }
}
#[doc = "Pixel format of the framebuffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRITEFORMAT10_AW {
    #[doc = "0: 8bpp a(8)0"]
    _00 = 0,
    #[doc = "1: 16bpp RGB(565)"]
    _01 = 1,
    #[doc = "2: 32bpp aRGB(8888)"]
    _10 = 2,
    #[doc = "3: 16bpp aRGB(4444)"]
    _11 = 3,
}
impl From<WRITEFORMAT10_AW> for u8 {
    #[inline(always)]
    fn from(variant: WRITEFORMAT10_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `WRITEFORMAT10` writer - Pixel format of the framebuffer"]
pub type WRITEFORMAT10_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONTROL2_SPEC, u8, WRITEFORMAT10_AW, 2, O>;
impl<'a, const O: u8> WRITEFORMAT10_W<'a, O> {
    #[doc = "8bpp a(8)0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WRITEFORMAT10_AW::_00)
    }
    #[doc = "16bpp RGB(565)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WRITEFORMAT10_AW::_01)
    }
    #[doc = "32bpp aRGB(8888)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WRITEFORMAT10_AW::_10)
    }
    #[doc = "16bpp aRGB(4444)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WRITEFORMAT10_AW::_11)
    }
}
#[doc = "Writeback alpha source for framebufferSet the 'alpha source' for the framebuffer(USEACB = 0)Blend alpha in color 2 instead of framebuffer alpha((USEACB = 1))In not alpha channel blending mode (USEACB = 0):Set the 'alpha source' for the framebuffer.In alpha channel blending mode (USEACB = 1):Blend alpha in color 2 instead of framebuffer alpha00B: BC2A = 1: use alpha from framebuffer as destination (DST_A)else: BC2A = 0: use alpha in color 2 as destination (DST_A)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRITEALPHA_AW {
    #[doc = "0: use alpha from color 2"]
    _00 = 0,
    #[doc = "1: use source alpha (pixel coverage)"]
    _01 = 1,
    #[doc = "2: use 0.0 as alpha"]
    _10 = 2,
    #[doc = "3: use alpha from framebuffer"]
    _11 = 3,
}
impl From<WRITEALPHA_AW> for u8 {
    #[inline(always)]
    fn from(variant: WRITEALPHA_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `WRITEALPHA` writer - Writeback alpha source for framebufferSet the 'alpha source' for the framebuffer(USEACB = 0)Blend alpha in color 2 instead of framebuffer alpha((USEACB = 1))In not alpha channel blending mode (USEACB = 0):Set the 'alpha source' for the framebuffer.In alpha channel blending mode (USEACB = 1):Blend alpha in color 2 instead of framebuffer alpha00B: BC2A = 1: use alpha from framebuffer as destination (DST_A)else: BC2A = 0: use alpha in color 2 as destination (DST_A)"]
pub type WRITEALPHA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONTROL2_SPEC, u8, WRITEALPHA_AW, 2, O>;
impl<'a, const O: u8> WRITEALPHA_W<'a, O> {
    #[doc = "use alpha from color 2"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WRITEALPHA_AW::_00)
    }
    #[doc = "use source alpha (pixel coverage)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WRITEALPHA_AW::_01)
    }
    #[doc = "use 0.0 as alpha"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WRITEALPHA_AW::_10)
    }
    #[doc = "use alpha from framebuffer"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WRITEALPHA_AW::_11)
    }
}
#[doc = "RLE enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLEENABLE_AW {
    #[doc = "0: RLE disabled"]
    _0 = 0,
    #[doc = "1: RLE enabled"]
    _1 = 1,
}
impl From<RLEENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: RLEENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLEENABLE` writer - RLE enable"]
pub type RLEENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, RLEENABLE_AW, O>;
impl<'a, const O: u8> RLEENABLE_W<'a, O> {
    #[doc = "RLE disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RLEENABLE_AW::_0)
    }
    #[doc = "RLE enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RLEENABLE_AW::_1)
    }
}
#[doc = "CLUT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLUTENABLE_AW {
    #[doc = "0: CLUT disabled"]
    _0 = 0,
    #[doc = "1: CLUT enabled"]
    _1 = 1,
}
impl From<CLUTENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: CLUTENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLUTENABLE` writer - CLUT enable"]
pub type CLUTENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, CLUTENABLE_AW, O>;
impl<'a, const O: u8> CLUTENABLE_W<'a, O> {
    #[doc = "CLUT disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLUTENABLE_AW::_0)
    }
    #[doc = "CLUT enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLUTENABLE_AW::_1)
    }
}
#[doc = "color keying enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLKEYENABLE_AW {
    #[doc = "0: color keying disabled"]
    _0 = 0,
    #[doc = "1: color keying enabled"]
    _1 = 1,
}
impl From<COLKEYENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: COLKEYENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLKEYENABLE` writer - color keying enable"]
pub type COLKEYENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL2_SPEC, COLKEYENABLE_AW, O>;
impl<'a, const O: u8> COLKEYENABLE_W<'a, O> {
    #[doc = "color keying disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COLKEYENABLE_AW::_0)
    }
    #[doc = "color keying enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COLKEYENABLE_AW::_1)
    }
}
#[doc = "Format of the CLUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLUTFORMAT_AW {
    #[doc = "0: aRGB(8888)"]
    _0 = 0,
    #[doc = "1: RGB(565)"]
    _1 = 1,
}
impl From<CLUTFORMAT_AW> for bool {
    #[inline(always)]
    fn from(variant: CLUTFORMAT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLUTFORMAT` writer - Format of the CLUT"]
pub type CLUTFORMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, CLUTFORMAT_AW, O>;
impl<'a, const O: u8> CLUTFORMAT_W<'a, O> {
    #[doc = "aRGB(8888)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLUTFORMAT_AW::_0)
    }
    #[doc = "RGB(565)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLUTFORMAT_AW::_1)
    }
}
#[doc = "Blend source factor inverted in alpha channel (USEACB = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSIA_AW {
    #[doc = "0: use blend factor as specified through BSFA"]
    _0 = 0,
    #[doc = "1: invert blend source factor (1-x)"]
    _1 = 1,
}
impl From<BSIA_AW> for bool {
    #[inline(always)]
    fn from(variant: BSIA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSIA` writer - Blend source factor inverted in alpha channel (USEACB = 1)"]
pub type BSIA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, BSIA_AW, O>;
impl<'a, const O: u8> BSIA_W<'a, O> {
    #[doc = "use blend factor as specified through BSFA"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSIA_AW::_0)
    }
    #[doc = "invert blend source factor (1-x)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSIA_AW::_1)
    }
}
#[doc = "Blend destination factor inverted in alpha channel (USEACB = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDIA_AW {
    #[doc = "0: use blend factor as specified through BDFA"]
    _0 = 0,
    #[doc = "1: invert blend destination factor (1-x)"]
    _1 = 1,
}
impl From<BDIA_AW> for bool {
    #[inline(always)]
    fn from(variant: BDIA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDIA` writer - Blend destination factor inverted in alpha channel (USEACB = 1)"]
pub type BDIA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, BDIA_AW, O>;
impl<'a, const O: u8> BDIA_W<'a, O> {
    #[doc = "use blend factor as specified through BDFA"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BDIA_AW::_0)
    }
    #[doc = "invert blend destination factor (1-x)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BDIA_AW::_1)
    }
}
#[doc = "Texel width for RLE unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RLEPIXELWIDTH_AW {
    #[doc = "0: 1 byte per texel"]
    _00 = 0,
    #[doc = "1: 2 byte per texel"]
    _01 = 1,
    #[doc = "2: 3 byte per texel"]
    _10 = 2,
    #[doc = "3: 4 byte per texel"]
    _11 = 3,
}
impl From<RLEPIXELWIDTH_AW> for u8 {
    #[inline(always)]
    fn from(variant: RLEPIXELWIDTH_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `RLEPIXELWIDTH` writer - Texel width for RLE unit"]
pub type RLEPIXELWIDTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONTROL2_SPEC, u8, RLEPIXELWIDTH_AW, 2, O>;
impl<'a, const O: u8> RLEPIXELWIDTH_W<'a, O> {
    #[doc = "1 byte per texel"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RLEPIXELWIDTH_AW::_00)
    }
    #[doc = "2 byte per texel"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RLEPIXELWIDTH_AW::_01)
    }
    #[doc = "3 byte per texel"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RLEPIXELWIDTH_AW::_10)
    }
    #[doc = "4 byte per texel"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RLEPIXELWIDTH_AW::_11)
    }
}
impl W {
    #[doc = "Bit 0 - Pixel source is a pattern color (blend of COLOR1 and COLOR2 depending on PATTERN and pattern index)"]
    #[inline(always)]
    #[must_use]
    pub fn patternenable(&mut self) -> PATTERNENABLE_W<0> {
        PATTERNENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Pixel source is read from texture and used as an alpha to blend between COLOR1 and COLOR2"]
    #[inline(always)]
    #[must_use]
    pub fn textureenable(&mut self) -> TEXTUREENABLE_W<1> {
        TEXTUREENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Limiter 5 is used as pattern index instead of the default U limiter.Limiter 5 can be combined with limiter 6 to form a quadratic limiter which can be used to make quadratic pattern functions to draw radial patterns."]
    #[inline(always)]
    #[must_use]
    pub fn patternsourcel5(&mut self) -> PATTERNSOURCEL5_W<2> {
        PATTERNSOURCEL5_W::new(self)
    }
    #[doc = "Bit 3 - Alpha blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn useacb(&mut self) -> USEACB_W<3> {
        USEACB_W::new(self)
    }
    #[doc = "Bits 4:5 - Bit 4 and 3 of the texture buffer format.See READFORMAT above for description"]
    #[inline(always)]
    #[must_use]
    pub fn readformat32(&mut self) -> READFORMAT32_W<4> {
        READFORMAT32_W::new(self)
    }
    #[doc = "Bit 6 - Blend source factor for alpha channel in alpha channel blending mode (USEACB = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn bsfa(&mut self) -> BSFA_W<6> {
        BSFA_W::new(self)
    }
    #[doc = "Bit 7 - Blend destinetion factor for alpha channel in alpha channel blending mode (USEACB = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn bdfa(&mut self) -> BDFA_W<7> {
        BDFA_W::new(self)
    }
    #[doc = "Bit 8 - Bit 3 of framebuffer pixel formatSee WRITEFORMAT above description."]
    #[inline(always)]
    #[must_use]
    pub fn writeformat2(&mut self) -> WRITEFORMAT2_W<8> {
        WRITEFORMAT2_W::new(self)
    }
    #[doc = "Bit 9 - Blend source factorsrc factor is alpha (factor is 1 per default)"]
    #[inline(always)]
    #[must_use]
    pub fn bsf(&mut self) -> BSF_W<9> {
        BSF_W::new(self)
    }
    #[doc = "Bit 10 - Blend destination factordst factor is alpha (factor is 1 per default)"]
    #[inline(always)]
    #[must_use]
    pub fn bdf(&mut self) -> BDF_W<10> {
        BDF_W::new(self)
    }
    #[doc = "Bit 11 - Blend source factor is invertedsrc factor will be inverted (meaning 1-a or 1-1 depending on BSF)"]
    #[inline(always)]
    #[must_use]
    pub fn bsi(&mut self) -> BSI_W<11> {
        BSI_W::new(self)
    }
    #[doc = "Bit 12 - Blend destination factor is inverteddst factor will be inverted (meaning 1-a or 1-1 depending on BDF)"]
    #[inline(always)]
    #[must_use]
    pub fn bdi(&mut self) -> BDI_W<12> {
        BDI_W::new(self)
    }
    #[doc = "Bit 13 - Blend color 2 instead of framebuffer pixel"]
    #[inline(always)]
    #[must_use]
    pub fn bc2(&mut self) -> BC2_W<13> {
        BC2_W::new(self)
    }
    #[doc = "Bit 14 - Calculating U limiter outside use textureThe bit describes what happens if the U limiter (x direction in texture space) calculates a U value outside of the used texture"]
    #[inline(always)]
    #[must_use]
    pub fn textureclampx(&mut self) -> TEXTURECLAMPX_W<14> {
        TEXTURECLAMPX_W::new(self)
    }
    #[doc = "Bit 15 - Calculating V limiter outside use textureThe bit describes what happens if the V limiter (y direction in texture space) calculates a V value outside of the used texture"]
    #[inline(always)]
    #[must_use]
    pub fn textureclampy(&mut self) -> TEXTURECLAMPY_W<15> {
        TEXTURECLAMPY_W::new(self)
    }
    #[doc = "Bit 16 - Linear filtering on texture U axis"]
    #[inline(always)]
    #[must_use]
    pub fn texturefilterx(&mut self) -> TEXTUREFILTERX_W<16> {
        TEXTUREFILTERX_W::new(self)
    }
    #[doc = "Bit 17 - Linear filtering on texture V axis"]
    #[inline(always)]
    #[must_use]
    pub fn texturefiltery(&mut self) -> TEXTUREFILTERY_W<17> {
        TEXTUREFILTERY_W::new(self)
    }
    #[doc = "Bits 18:19 - Pixel format of the texture buffer{READFORMAT32,READFORMAT10}0000: 8 bpp a(8)0001: 16 bpp RGB(565)0010: 32 bpp aRGB(8888)0011: 16 bpp aRGB(4444)0100: 16 bpp aRGB(1555)0101: 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color1001: 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance1010: 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance1011: 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance 1100: 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance"]
    #[inline(always)]
    #[must_use]
    pub fn readformat10(&mut self) -> READFORMAT10_W<18> {
        READFORMAT10_W::new(self)
    }
    #[doc = "Bits 20:21 - Pixel format of the framebuffer"]
    #[inline(always)]
    #[must_use]
    pub fn writeformat10(&mut self) -> WRITEFORMAT10_W<20> {
        WRITEFORMAT10_W::new(self)
    }
    #[doc = "Bits 22:23 - Writeback alpha source for framebufferSet the 'alpha source' for the framebuffer(USEACB = 0)Blend alpha in color 2 instead of framebuffer alpha((USEACB = 1))In not alpha channel blending mode (USEACB = 0):Set the 'alpha source' for the framebuffer.In alpha channel blending mode (USEACB = 1):Blend alpha in color 2 instead of framebuffer alpha00B: BC2A = 1: use alpha from framebuffer as destination (DST_A)else: BC2A = 0: use alpha in color 2 as destination (DST_A)"]
    #[inline(always)]
    #[must_use]
    pub fn writealpha(&mut self) -> WRITEALPHA_W<22> {
        WRITEALPHA_W::new(self)
    }
    #[doc = "Bit 24 - RLE enable"]
    #[inline(always)]
    #[must_use]
    pub fn rleenable(&mut self) -> RLEENABLE_W<24> {
        RLEENABLE_W::new(self)
    }
    #[doc = "Bit 25 - CLUT enable"]
    #[inline(always)]
    #[must_use]
    pub fn clutenable(&mut self) -> CLUTENABLE_W<25> {
        CLUTENABLE_W::new(self)
    }
    #[doc = "Bit 26 - color keying enable"]
    #[inline(always)]
    #[must_use]
    pub fn colkeyenable(&mut self) -> COLKEYENABLE_W<26> {
        COLKEYENABLE_W::new(self)
    }
    #[doc = "Bit 27 - Format of the CLUT"]
    #[inline(always)]
    #[must_use]
    pub fn clutformat(&mut self) -> CLUTFORMAT_W<27> {
        CLUTFORMAT_W::new(self)
    }
    #[doc = "Bit 28 - Blend source factor inverted in alpha channel (USEACB = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn bsia(&mut self) -> BSIA_W<28> {
        BSIA_W::new(self)
    }
    #[doc = "Bit 29 - Blend destination factor inverted in alpha channel (USEACB = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn bdia(&mut self) -> BDIA_W<29> {
        BDIA_W::new(self)
    }
    #[doc = "Bits 30:31 - Texel width for RLE unit"]
    #[inline(always)]
    #[must_use]
    pub fn rlepixelwidth(&mut self) -> RLEPIXELWIDTH_W<30> {
        RLEPIXELWIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Surface Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control2](index.html) module"]
pub struct CONTROL2_SPEC;
impl crate::RegisterSpec for CONTROL2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [control2::W](W) writer structure"]
impl crate::Writable for CONTROL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL2 to value 0"]
impl crate::Resettable for CONTROL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
