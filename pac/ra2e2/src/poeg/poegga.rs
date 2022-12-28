#[doc = "Register `POEGGA` reader"]
pub struct R(crate::R<POEGGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POEGGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POEGGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POEGGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POEGGA` writer"]
pub struct W(crate::W<POEGGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POEGGA_SPEC>;
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
impl From<crate::W<POEGGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POEGGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIDF` reader - Port Input Detection Flag"]
pub type PIDF_R = crate::BitReader<PIDF_A>;
#[doc = "Port Input Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDF_A {
    #[doc = "0: No output-disable request from the GTETRGn pin occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from the GTETRGn pin occurred."]
    _1 = 1,
}
impl From<PIDF_A> for bool {
    #[inline(always)]
    fn from(variant: PIDF_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDF_A {
        match self.bits {
            false => PIDF_A::_0,
            true => PIDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDF_A::_1
    }
}
#[doc = "Field `PIDF` writer - Port Input Detection Flag"]
pub type PIDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGA_SPEC, PIDF_A, O>;
impl<'a, const O: u8> PIDF_W<'a, O> {
    #[doc = "No output-disable request from the GTETRGn pin occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIDF_A::_0)
    }
    #[doc = "Output-disable request from the GTETRGn pin occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIDF_A::_1)
    }
}
#[doc = "Field `IOCF` reader - Detection Flag for GPT Output-Disable Request"]
pub type IOCF_R = crate::BitReader<IOCF_A>;
#[doc = "Detection Flag for GPT Output-Disable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCF_A {
    #[doc = "0: No output-disable request from GPT occurred."]
    _0 = 0,
    #[doc = "1: Output-disable request from GPT occurred."]
    _1 = 1,
}
impl From<IOCF_A> for bool {
    #[inline(always)]
    fn from(variant: IOCF_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCF_A {
        match self.bits {
            false => IOCF_A::_0,
            true => IOCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOCF_A::_1
    }
}
#[doc = "Field `IOCF` writer - Detection Flag for GPT Output-Disable Request"]
pub type IOCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGA_SPEC, IOCF_A, O>;
impl<'a, const O: u8> IOCF_W<'a, O> {
    #[doc = "No output-disable request from GPT occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IOCF_A::_0)
    }
    #[doc = "Output-disable request from GPT occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IOCF_A::_1)
    }
}
#[doc = "Field `SSF` reader - Software Stop Flag"]
pub type SSF_R = crate::BitReader<SSF_A>;
#[doc = "Software Stop Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSF_A {
    #[doc = "0: No output-disable request from software occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from software occurred"]
    _1 = 1,
}
impl From<SSF_A> for bool {
    #[inline(always)]
    fn from(variant: SSF_A) -> Self {
        variant as u8 != 0
    }
}
impl SSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSF_A {
        match self.bits {
            false => SSF_A::_0,
            true => SSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSF_A::_1
    }
}
#[doc = "Field `SSF` writer - Software Stop Flag"]
pub type SSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGA_SPEC, SSF_A, O>;
impl<'a, const O: u8> SSF_W<'a, O> {
    #[doc = "No output-disable request from software occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSF_A::_0)
    }
    #[doc = "Output-disable request from software occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSF_A::_1)
    }
}
#[doc = "Field `PIDE` reader - Port Input Detection Enable"]
pub type PIDE_R = crate::BitReader<PIDE_A>;
#[doc = "Port Input Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDE_A {
    #[doc = "0: Disable output-disable requests from the GTETRGn pins"]
    _0 = 0,
    #[doc = "1: Enable output-disable requests from the GTETRGn pins"]
    _1 = 1,
}
impl From<PIDE_A> for bool {
    #[inline(always)]
    fn from(variant: PIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDE_A {
        match self.bits {
            false => PIDE_A::_0,
            true => PIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDE_A::_1
    }
}
#[doc = "Field `PIDE` writer - Port Input Detection Enable"]
pub type PIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGA_SPEC, PIDE_A, O>;
impl<'a, const O: u8> PIDE_W<'a, O> {
    #[doc = "Disable output-disable requests from the GTETRGn pins"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIDE_A::_0)
    }
    #[doc = "Enable output-disable requests from the GTETRGn pins"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIDE_A::_1)
    }
}
#[doc = "Field `IOCE` reader - Enable for GPT Output-Disable Request"]
pub type IOCE_R = crate::BitReader<IOCE_A>;
#[doc = "Enable for GPT Output-Disable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCE_A {
    #[doc = "0: Disable output-disable requests from GPT"]
    _0 = 0,
    #[doc = "1: Enable output-disable requests from GPT"]
    _1 = 1,
}
impl From<IOCE_A> for bool {
    #[inline(always)]
    fn from(variant: IOCE_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCE_A {
        match self.bits {
            false => IOCE_A::_0,
            true => IOCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOCE_A::_1
    }
}
#[doc = "Field `IOCE` writer - Enable for GPT Output-Disable Request"]
pub type IOCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGA_SPEC, IOCE_A, O>;
impl<'a, const O: u8> IOCE_W<'a, O> {
    #[doc = "Disable output-disable requests from GPT"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IOCE_A::_0)
    }
    #[doc = "Enable output-disable requests from GPT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IOCE_A::_1)
    }
}
#[doc = "Field `ST` reader - GTETRGn Input Status Flag"]
pub type ST_R = crate::BitReader<ST_A>;
#[doc = "GTETRGn Input Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_A {
    #[doc = "0: GTETRGn input after filtering was 0"]
    _0 = 0,
    #[doc = "1: GTETRGn input after filtering was 1"]
    _1 = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
impl ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::_0,
            true => ST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ST_A::_1
    }
}
#[doc = "Field `INV` reader - GTETRGn Input Reverse"]
pub type INV_R = crate::BitReader<INV_A>;
#[doc = "GTETRGn Input Reverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    #[doc = "0: Input GTETRGn as-is"]
    _0 = 0,
    #[doc = "1: Input GTETRGn in reverse"]
    _1 = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::_0,
            true => INV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
}
#[doc = "Field `INV` writer - GTETRGn Input Reverse"]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGA_SPEC, INV_A, O>;
impl<'a, const O: u8> INV_W<'a, O> {
    #[doc = "Input GTETRGn as-is"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV_A::_0)
    }
    #[doc = "Input GTETRGn in reverse"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV_A::_1)
    }
}
#[doc = "Field `NFEN` reader - Noise Filter Enable"]
pub type NFEN_R = crate::BitReader<NFEN_A>;
#[doc = "Noise Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFEN_A {
    #[doc = "0: Disable noise filtering"]
    _0 = 0,
    #[doc = "1: Enable noise filtering"]
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
#[doc = "Field `NFEN` writer - Noise Filter Enable"]
pub type NFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGA_SPEC, NFEN_A, O>;
impl<'a, const O: u8> NFEN_W<'a, O> {
    #[doc = "Disable noise filtering"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFEN_A::_0)
    }
    #[doc = "Enable noise filtering"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFEN_A::_1)
    }
}
#[doc = "Field `NFCS` reader - Noise Filter Clock Select"]
pub type NFCS_R = crate::FieldReader<u8, NFCS_A>;
#[doc = "Noise Filter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    #[doc = "0: Sample GTETRGn pin input level three times every PCLKB"]
    _00 = 0,
    #[doc = "1: Sample GTETRGn pin input level three times every PCLKB/8"]
    _01 = 1,
    #[doc = "2: Sample GTETRGn pin input level three times every PCLKB/32"]
    _10 = 2,
    #[doc = "3: Sample GTETRGn pin input level three times every PCLKB/128"]
    _11 = 3,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl NFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFCS_A {
        match self.bits {
            0 => NFCS_A::_00,
            1 => NFCS_A::_01,
            2 => NFCS_A::_10,
            3 => NFCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCS_A::_11
    }
}
#[doc = "Field `NFCS` writer - Noise Filter Clock Select"]
pub type NFCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, POEGGA_SPEC, u8, NFCS_A, 2, O>;
impl<'a, const O: u8> NFCS_W<'a, O> {
    #[doc = "Sample GTETRGn pin input level three times every PCLKB"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NFCS_A::_00)
    }
    #[doc = "Sample GTETRGn pin input level three times every PCLKB/8"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NFCS_A::_01)
    }
    #[doc = "Sample GTETRGn pin input level three times every PCLKB/32"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NFCS_A::_10)
    }
    #[doc = "Sample GTETRGn pin input level three times every PCLKB/128"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NFCS_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(&self) -> PIDF_R {
        PIDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Detection Flag for GPT Output-Disable Request"]
    #[inline(always)]
    pub fn iocf(&self) -> IOCF_R {
        IOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Stop Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Input Detection Enable"]
    #[inline(always)]
    pub fn pide(&self) -> PIDE_R {
        PIDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable for GPT Output-Disable Request"]
    #[inline(always)]
    pub fn ioce(&self) -> IOCE_R {
        IOCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - GTETRGn Input Status Flag"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - GTETRGn Input Reverse"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Noise Filter Enable"]
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Port Input Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pidf(&mut self) -> PIDF_W<0> {
        PIDF_W::new(self)
    }
    #[doc = "Bit 1 - Detection Flag for GPT Output-Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn iocf(&mut self) -> IOCF_W<1> {
        IOCF_W::new(self)
    }
    #[doc = "Bit 3 - Software Stop Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssf(&mut self) -> SSF_W<3> {
        SSF_W::new(self)
    }
    #[doc = "Bit 4 - Port Input Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pide(&mut self) -> PIDE_W<4> {
        PIDE_W::new(self)
    }
    #[doc = "Bit 5 - Enable for GPT Output-Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn ioce(&mut self) -> IOCE_W<5> {
        IOCE_W::new(self)
    }
    #[doc = "Bit 28 - GTETRGn Input Reverse"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<28> {
        INV_W::new(self)
    }
    #[doc = "Bit 29 - Noise Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfen(&mut self) -> NFEN_W<29> {
        NFEN_W::new(self)
    }
    #[doc = "Bits 30:31 - Noise Filter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcs(&mut self) -> NFCS_W<30> {
        NFCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POEG Group A Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poegga](index.html) module"]
pub struct POEGGA_SPEC;
impl crate::RegisterSpec for POEGGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poegga::R](R) reader structure"]
impl crate::Readable for POEGGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poegga::W](W) writer structure"]
impl crate::Writable for POEGGA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POEGGA to value 0"]
impl crate::Resettable for POEGGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
