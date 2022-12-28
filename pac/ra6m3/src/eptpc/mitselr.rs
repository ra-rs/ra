#[doc = "Register `MITSELR` reader"]
pub struct R(crate::R<MITSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MITSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MITSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MITSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MITSELR` writer"]
pub struct W(crate::W<MITSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MITSELR_SPEC>;
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
impl From<crate::W<MITSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MITSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MINTEN0` reader - Pulse Output Timer 0 MINT Interrupt Output Enable"]
pub type MINTEN0_R = crate::BitReader<MINTEN0_A>;
#[doc = "Pulse Output Timer 0 MINT Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN0_A {
    #[doc = "0: Output of rising edges by pulse output timer 0 is not reflected by the MIESR.CYC0 flag as a MINT interrupt source."]
    _0 = 0,
    #[doc = "1: Output of rising edges by pulse output timer 0 is reflected by the MIESR.CYC0 flag as a MINT interrupt source."]
    _1 = 1,
}
impl From<MINTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl MINTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MINTEN0_A {
        match self.bits {
            false => MINTEN0_A::_0,
            true => MINTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN0_A::_1
    }
}
#[doc = "Field `MINTEN0` writer - Pulse Output Timer 0 MINT Interrupt Output Enable"]
pub type MINTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MITSELR_SPEC, MINTEN0_A, O>;
impl<'a, const O: u8> MINTEN0_W<'a, O> {
    #[doc = "Output of rising edges by pulse output timer 0 is not reflected by the MIESR.CYC0 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MINTEN0_A::_0)
    }
    #[doc = "Output of rising edges by pulse output timer 0 is reflected by the MIESR.CYC0 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MINTEN0_A::_1)
    }
}
#[doc = "Field `MINTEN1` reader - Pulse Output Timer 1 MINT Interrupt Output Enable"]
pub type MINTEN1_R = crate::BitReader<MINTEN1_A>;
#[doc = "Pulse Output Timer 1 MINT Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN1_A {
    #[doc = "0: Output of rising edges by pulse output timer 1 is not reflected by the MIESR.CYC1 flag as a MINT interrupt source."]
    _0 = 0,
    #[doc = "1: Output of rising edges by pulse output timer 1 is reflected by the MIESR.CYC1 flag as a MINT interrupt source."]
    _1 = 1,
}
impl From<MINTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl MINTEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MINTEN1_A {
        match self.bits {
            false => MINTEN1_A::_0,
            true => MINTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN1_A::_1
    }
}
#[doc = "Field `MINTEN1` writer - Pulse Output Timer 1 MINT Interrupt Output Enable"]
pub type MINTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MITSELR_SPEC, MINTEN1_A, O>;
impl<'a, const O: u8> MINTEN1_W<'a, O> {
    #[doc = "Output of rising edges by pulse output timer 1 is not reflected by the MIESR.CYC1 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MINTEN1_A::_0)
    }
    #[doc = "Output of rising edges by pulse output timer 1 is reflected by the MIESR.CYC1 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MINTEN1_A::_1)
    }
}
#[doc = "Field `MINTEN2` reader - Pulse Output Timer 2 MINT Interrupt Output Enable"]
pub type MINTEN2_R = crate::BitReader<MINTEN2_A>;
#[doc = "Pulse Output Timer 2 MINT Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN2_A {
    #[doc = "0: Output of rising edges by pulse output timer 2 is not reflected by the MIESR.CYC2 flag as a MINT interrupt source."]
    _0 = 0,
    #[doc = "1: Output of rising edges by pulse output timer 2 is reflected by the MIESR.CYC2 flag as a MINT interrupt source."]
    _1 = 1,
}
impl From<MINTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl MINTEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MINTEN2_A {
        match self.bits {
            false => MINTEN2_A::_0,
            true => MINTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN2_A::_1
    }
}
#[doc = "Field `MINTEN2` writer - Pulse Output Timer 2 MINT Interrupt Output Enable"]
pub type MINTEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MITSELR_SPEC, MINTEN2_A, O>;
impl<'a, const O: u8> MINTEN2_W<'a, O> {
    #[doc = "Output of rising edges by pulse output timer 2 is not reflected by the MIESR.CYC2 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MINTEN2_A::_0)
    }
    #[doc = "Output of rising edges by pulse output timer 2 is reflected by the MIESR.CYC2 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MINTEN2_A::_1)
    }
}
#[doc = "Field `MINTEN3` reader - Pulse Output Timer 3 MINT Interrupt Output Enable"]
pub type MINTEN3_R = crate::BitReader<MINTEN3_A>;
#[doc = "Pulse Output Timer 3 MINT Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN3_A {
    #[doc = "0: Output of rising edges by pulse output timer 3 is not reflected by the MIESR.CYC3 flag as a MINT interrupt source."]
    _0 = 0,
    #[doc = "1: Output of rising edges by pulse output timer 3 is reflected by the MIESR.CYC3 flag as a MINT interrupt source."]
    _1 = 1,
}
impl From<MINTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl MINTEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MINTEN3_A {
        match self.bits {
            false => MINTEN3_A::_0,
            true => MINTEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN3_A::_1
    }
}
#[doc = "Field `MINTEN3` writer - Pulse Output Timer 3 MINT Interrupt Output Enable"]
pub type MINTEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MITSELR_SPEC, MINTEN3_A, O>;
impl<'a, const O: u8> MINTEN3_W<'a, O> {
    #[doc = "Output of rising edges by pulse output timer 3 is not reflected by the MIESR.CYC3 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MINTEN3_A::_0)
    }
    #[doc = "Output of rising edges by pulse output timer 3 is reflected by the MIESR.CYC3 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MINTEN3_A::_1)
    }
}
#[doc = "Field `MINTEN4` reader - Pulse Output Timer 4 MINT Interrupt Output Enable"]
pub type MINTEN4_R = crate::BitReader<MINTEN4_A>;
#[doc = "Pulse Output Timer 4 MINT Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN4_A {
    #[doc = "0: Output of rising edges by pulse output timer 4 is not reflected by the MIESR.CYC4 flag as a MINT interrupt source."]
    _0 = 0,
    #[doc = "1: Output of rising edges by pulse output timer 4 is reflected by the MIESR.CYC4 flag as a MINT interrupt source."]
    _1 = 1,
}
impl From<MINTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl MINTEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MINTEN4_A {
        match self.bits {
            false => MINTEN4_A::_0,
            true => MINTEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN4_A::_1
    }
}
#[doc = "Field `MINTEN4` writer - Pulse Output Timer 4 MINT Interrupt Output Enable"]
pub type MINTEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MITSELR_SPEC, MINTEN4_A, O>;
impl<'a, const O: u8> MINTEN4_W<'a, O> {
    #[doc = "Output of rising edges by pulse output timer 4 is not reflected by the MIESR.CYC4 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MINTEN4_A::_0)
    }
    #[doc = "Output of rising edges by pulse output timer 4 is reflected by the MIESR.CYC4 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MINTEN4_A::_1)
    }
}
#[doc = "Field `MINTEN5` reader - Pulse Output Timer 5 MINT Interrupt Output Enable"]
pub type MINTEN5_R = crate::BitReader<MINTEN5_A>;
#[doc = "Pulse Output Timer 5 MINT Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN5_A {
    #[doc = "0: Output of rising edges by pulse output timer 5 is not reflected by the MIESR.CYC5 flag as a MINT interrupt source."]
    _0 = 0,
    #[doc = "1: Output of rising edges by pulse output timer 5 is reflected by the MIESR.CYC5 flag as a MINT interrupt source."]
    _1 = 1,
}
impl From<MINTEN5_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl MINTEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MINTEN5_A {
        match self.bits {
            false => MINTEN5_A::_0,
            true => MINTEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN5_A::_1
    }
}
#[doc = "Field `MINTEN5` writer - Pulse Output Timer 5 MINT Interrupt Output Enable"]
pub type MINTEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MITSELR_SPEC, MINTEN5_A, O>;
impl<'a, const O: u8> MINTEN5_W<'a, O> {
    #[doc = "Output of rising edges by pulse output timer 5 is not reflected by the MIESR.CYC5 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MINTEN5_A::_0)
    }
    #[doc = "Output of rising edges by pulse output timer 5 is reflected by the MIESR.CYC5 flag as a MINT interrupt source."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MINTEN5_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pulse Output Timer 0 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten0(&self) -> MINTEN0_R {
        MINTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten1(&self) -> MINTEN1_R {
        MINTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten2(&self) -> MINTEN2_R {
        MINTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten3(&self) -> MINTEN3_R {
        MINTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten4(&self) -> MINTEN4_R {
        MINTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten5(&self) -> MINTEN5_R {
        MINTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Output Timer 0 MINT Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minten0(&mut self) -> MINTEN0_W<0> {
        MINTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 MINT Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minten1(&mut self) -> MINTEN1_W<1> {
        MINTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 MINT Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minten2(&mut self) -> MINTEN2_W<2> {
        MINTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 MINT Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minten3(&mut self) -> MINTEN3_W<3> {
        MINTEN3_W::new(self)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 MINT Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minten4(&mut self) -> MINTEN4_W<4> {
        MINTEN4_W::new(self)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 MINT Interrupt Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minten5(&mut self) -> MINTEN5_W<5> {
        MINTEN5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MINT Interrupt Request Timer Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mitselr](index.html) module"]
pub struct MITSELR_SPEC;
impl crate::RegisterSpec for MITSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mitselr::R](R) reader structure"]
impl crate::Readable for MITSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mitselr::W](W) writer structure"]
impl crate::Writable for MITSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MITSELR to value 0"]
impl crate::Resettable for MITSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
