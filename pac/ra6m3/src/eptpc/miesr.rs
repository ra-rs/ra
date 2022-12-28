#[doc = "Register `MIESR` reader"]
pub struct R(crate::R<MIESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIESR` writer"]
pub struct W(crate::W<MIESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIESR_SPEC>;
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
impl From<crate::W<MIESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST` reader - STCA Status Flag"]
pub type ST_R = crate::BitReader<ST_A>;
#[doc = "STCA Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_A {
    #[doc = "0: No change in the state of the STCA module"]
    _0 = 0,
    #[doc = "1: A change in the state of the STCA module"]
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
#[doc = "Field `SY0` reader - SYNFP0 Status Flag"]
pub type SY0_R = crate::BitReader<SY0_A>;
#[doc = "SYNFP0 Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SY0_A {
    #[doc = "0: No change in the state of the SYNFP0 module"]
    _0 = 0,
    #[doc = "1: A change in the state of the SYNFP0 module"]
    _1 = 1,
}
impl From<SY0_A> for bool {
    #[inline(always)]
    fn from(variant: SY0_A) -> Self {
        variant as u8 != 0
    }
}
impl SY0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SY0_A {
        match self.bits {
            false => SY0_A::_0,
            true => SY0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SY0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SY0_A::_1
    }
}
#[doc = "Field `CYC0` reader - Pulse Output Timer 0 Rising Edge Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type CYC0_R = crate::BitReader<CYC0_A>;
#[doc = "Pulse Output Timer 0 Rising Edge Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC0_A {
    #[doc = "0: A rising edge in the periodic pulse signal from pulse output timer 0 is not detected."]
    _0 = 0,
    #[doc = "1: A rising edge in the periodic pulse signal from pulse output timer 0 is detected."]
    _1 = 1,
}
impl From<CYC0_A> for bool {
    #[inline(always)]
    fn from(variant: CYC0_A) -> Self {
        variant as u8 != 0
    }
}
impl CYC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYC0_A {
        match self.bits {
            false => CYC0_A::_0,
            true => CYC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC0_A::_1
    }
}
#[doc = "Field `CYC0` writer - Pulse Output Timer 0 Rising Edge Detection Flag"]
pub type CYC0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MIESR_SPEC, CYC0_A, O>;
impl<'a, const O: u8> CYC0_W<'a, O> {
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 0 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYC0_A::_0)
    }
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 0 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYC0_A::_1)
    }
}
#[doc = "Field `CYC1` reader - Pulse Output Timer 1 Rising Edge Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type CYC1_R = crate::BitReader<CYC1_A>;
#[doc = "Pulse Output Timer 1 Rising Edge Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC1_A {
    #[doc = "0: A rising edge in the periodic pulse signal from pulse output timer 1 is not detected."]
    _0 = 0,
    #[doc = "1: A rising edge in the periodic pulse signal from pulse output timer 1 is detected."]
    _1 = 1,
}
impl From<CYC1_A> for bool {
    #[inline(always)]
    fn from(variant: CYC1_A) -> Self {
        variant as u8 != 0
    }
}
impl CYC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYC1_A {
        match self.bits {
            false => CYC1_A::_0,
            true => CYC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC1_A::_1
    }
}
#[doc = "Field `CYC1` writer - Pulse Output Timer 1 Rising Edge Detection Flag"]
pub type CYC1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MIESR_SPEC, CYC1_A, O>;
impl<'a, const O: u8> CYC1_W<'a, O> {
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 1 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYC1_A::_0)
    }
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 1 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYC1_A::_1)
    }
}
#[doc = "Field `CYC2` reader - Pulse Output Timer 2 Rising Edge Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type CYC2_R = crate::BitReader<CYC2_A>;
#[doc = "Pulse Output Timer 2 Rising Edge Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC2_A {
    #[doc = "0: A rising edge in the periodic pulse signal from pulse output timer 2 is not detected."]
    _0 = 0,
    #[doc = "1: A rising edge in the periodic pulse signal from pulse output timer 2 is detected."]
    _1 = 1,
}
impl From<CYC2_A> for bool {
    #[inline(always)]
    fn from(variant: CYC2_A) -> Self {
        variant as u8 != 0
    }
}
impl CYC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYC2_A {
        match self.bits {
            false => CYC2_A::_0,
            true => CYC2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC2_A::_1
    }
}
#[doc = "Field `CYC2` writer - Pulse Output Timer 2 Rising Edge Detection Flag"]
pub type CYC2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MIESR_SPEC, CYC2_A, O>;
impl<'a, const O: u8> CYC2_W<'a, O> {
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 2 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYC2_A::_0)
    }
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 2 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYC2_A::_1)
    }
}
#[doc = "Field `CYC3` reader - Pulse Output Timer 3 Rising Edge Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type CYC3_R = crate::BitReader<CYC3_A>;
#[doc = "Pulse Output Timer 3 Rising Edge Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC3_A {
    #[doc = "0: A rising edge in the periodic pulse signal from pulse output timer 3 is not detected."]
    _0 = 0,
    #[doc = "1: A rising edge in the periodic pulse signal from pulse output timer 3 is detected."]
    _1 = 1,
}
impl From<CYC3_A> for bool {
    #[inline(always)]
    fn from(variant: CYC3_A) -> Self {
        variant as u8 != 0
    }
}
impl CYC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYC3_A {
        match self.bits {
            false => CYC3_A::_0,
            true => CYC3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC3_A::_1
    }
}
#[doc = "Field `CYC3` writer - Pulse Output Timer 3 Rising Edge Detection Flag"]
pub type CYC3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MIESR_SPEC, CYC3_A, O>;
impl<'a, const O: u8> CYC3_W<'a, O> {
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 3 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYC3_A::_0)
    }
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 3 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYC3_A::_1)
    }
}
#[doc = "Field `CYC4` reader - Pulse Output Timer 4 Rising Edge Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type CYC4_R = crate::BitReader<CYC4_A>;
#[doc = "Pulse Output Timer 4 Rising Edge Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC4_A {
    #[doc = "0: A rising edge in the periodic pulse signal from pulse output timer 4 is not detected."]
    _0 = 0,
    #[doc = "1: A rising edge in the periodic pulse signal from pulse output timer 4 is detected."]
    _1 = 1,
}
impl From<CYC4_A> for bool {
    #[inline(always)]
    fn from(variant: CYC4_A) -> Self {
        variant as u8 != 0
    }
}
impl CYC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYC4_A {
        match self.bits {
            false => CYC4_A::_0,
            true => CYC4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC4_A::_1
    }
}
#[doc = "Field `CYC4` writer - Pulse Output Timer 4 Rising Edge Detection Flag"]
pub type CYC4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MIESR_SPEC, CYC4_A, O>;
impl<'a, const O: u8> CYC4_W<'a, O> {
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 4 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYC4_A::_0)
    }
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 4 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYC4_A::_1)
    }
}
#[doc = "Field `CYC5` reader - Pulse Output Timer 5 Rising Edge Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type CYC5_R = crate::BitReader<CYC5_A>;
#[doc = "Pulse Output Timer 5 Rising Edge Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC5_A {
    #[doc = "0: A rising edge in the periodic pulse signal from pulse output timer 5 is not detected."]
    _0 = 0,
    #[doc = "1: A rising edge in the periodic pulse signal from pulse output timer 5 is detected."]
    _1 = 1,
}
impl From<CYC5_A> for bool {
    #[inline(always)]
    fn from(variant: CYC5_A) -> Self {
        variant as u8 != 0
    }
}
impl CYC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYC5_A {
        match self.bits {
            false => CYC5_A::_0,
            true => CYC5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC5_A::_1
    }
}
#[doc = "Field `CYC5` writer - Pulse Output Timer 5 Rising Edge Detection Flag"]
pub type CYC5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MIESR_SPEC, CYC5_A, O>;
impl<'a, const O: u8> CYC5_W<'a, O> {
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 5 is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CYC5_A::_0)
    }
    #[doc = "A rising edge in the periodic pulse signal from pulse output timer 5 is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CYC5_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - STCA Status Flag"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNFP0 Status Flag"]
    #[inline(always)]
    pub fn sy0(&self) -> SY0_R {
        SY0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Pulse Output Timer 0 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc0(&self) -> CYC0_R {
        CYC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pulse Output Timer 1 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc1(&self) -> CYC1_R {
        CYC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pulse Output Timer 2 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc2(&self) -> CYC2_R {
        CYC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pulse Output Timer 3 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc3(&self) -> CYC3_R {
        CYC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pulse Output Timer 4 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc4(&self) -> CYC4_R {
        CYC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pulse Output Timer 5 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc5(&self) -> CYC5_R {
        CYC5_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Pulse Output Timer 0 Rising Edge Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cyc0(&mut self) -> CYC0_W<16> {
        CYC0_W::new(self)
    }
    #[doc = "Bit 17 - Pulse Output Timer 1 Rising Edge Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cyc1(&mut self) -> CYC1_W<17> {
        CYC1_W::new(self)
    }
    #[doc = "Bit 18 - Pulse Output Timer 2 Rising Edge Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cyc2(&mut self) -> CYC2_W<18> {
        CYC2_W::new(self)
    }
    #[doc = "Bit 19 - Pulse Output Timer 3 Rising Edge Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cyc3(&mut self) -> CYC3_W<19> {
        CYC3_W::new(self)
    }
    #[doc = "Bit 20 - Pulse Output Timer 4 Rising Edge Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cyc4(&mut self) -> CYC4_W<20> {
        CYC4_W::new(self)
    }
    #[doc = "Bit 21 - Pulse Output Timer 5 Rising Edge Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cyc5(&mut self) -> CYC5_W<21> {
        CYC5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MINT Interrupt Source Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miesr](index.html) module"]
pub struct MIESR_SPEC;
impl crate::RegisterSpec for MIESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miesr::R](R) reader structure"]
impl crate::Readable for MIESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miesr::W](W) writer structure"]
impl crate::Writable for MIESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x003f_0000;
}
#[doc = "`reset()` method sets MIESR to value 0"]
impl crate::Resettable for MIESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
