#[doc = "Register `DPSIEGR2` reader"]
pub struct R(crate::R<DPSIEGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIEGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIEGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIEGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIEGR2` writer"]
pub struct W(crate::W<DPSIEGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIEGR2_SPEC>;
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
impl From<crate::W<DPSIEGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIEGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLVD1IEG` reader - LVD1 Edge Select"]
pub type DLVD1IEG_R = crate::BitReader<DLVD1IEG_A>;
#[doc = "LVD1 Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD1IEG_A {
    #[doc = "0: A cancel request is generated when VCC<Vdet1 (fall) is detected"]
    _0 = 0,
    #[doc = "1: A cancel request is generated when VCC>=Vdet1 (rise) is detected"]
    _1 = 1,
}
impl From<DLVD1IEG_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD1IEG_A) -> Self {
        variant as u8 != 0
    }
}
impl DLVD1IEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLVD1IEG_A {
        match self.bits {
            false => DLVD1IEG_A::_0,
            true => DLVD1IEG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD1IEG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD1IEG_A::_1
    }
}
#[doc = "Field `DLVD1IEG` writer - LVD1 Edge Select"]
pub type DLVD1IEG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR2_SPEC, DLVD1IEG_A, O>;
impl<'a, const O: u8> DLVD1IEG_W<'a, O> {
    #[doc = "A cancel request is generated when VCC<Vdet1 (fall) is detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLVD1IEG_A::_0)
    }
    #[doc = "A cancel request is generated when VCC>=Vdet1 (rise) is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLVD1IEG_A::_1)
    }
}
#[doc = "Field `DLVD2IEG` reader - LVD2 Edge Select"]
pub type DLVD2IEG_R = crate::BitReader<DLVD2IEG_A>;
#[doc = "LVD2 Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD2IEG_A {
    #[doc = "0: A cancel request is generated when VCC<Vdet2 (fall) is detected"]
    _0 = 0,
    #[doc = "1: A cancel request is generated when VCC>=Vdet2 (rise) is detected"]
    _1 = 1,
}
impl From<DLVD2IEG_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD2IEG_A) -> Self {
        variant as u8 != 0
    }
}
impl DLVD2IEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLVD2IEG_A {
        match self.bits {
            false => DLVD2IEG_A::_0,
            true => DLVD2IEG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD2IEG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD2IEG_A::_1
    }
}
#[doc = "Field `DLVD2IEG` writer - LVD2 Edge Select"]
pub type DLVD2IEG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR2_SPEC, DLVD2IEG_A, O>;
impl<'a, const O: u8> DLVD2IEG_W<'a, O> {
    #[doc = "A cancel request is generated when VCC<Vdet2 (fall) is detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLVD2IEG_A::_0)
    }
    #[doc = "A cancel request is generated when VCC>=Vdet2 (rise) is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLVD2IEG_A::_1)
    }
}
#[doc = "Field `DNMIEG` reader - NMI Pin Edge Select"]
pub type DNMIEG_R = crate::BitReader<DNMIEG_A>;
#[doc = "NMI Pin Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNMIEG_A {
    #[doc = "0: A cancel request is generated at a falling edge"]
    _0 = 0,
    #[doc = "1: A cancel request is generated at a rising edge"]
    _1 = 1,
}
impl From<DNMIEG_A> for bool {
    #[inline(always)]
    fn from(variant: DNMIEG_A) -> Self {
        variant as u8 != 0
    }
}
impl DNMIEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNMIEG_A {
        match self.bits {
            false => DNMIEG_A::_0,
            true => DNMIEG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DNMIEG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DNMIEG_A::_1
    }
}
#[doc = "Field `DNMIEG` writer - NMI Pin Edge Select"]
pub type DNMIEG_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIEGR2_SPEC, DNMIEG_A, O>;
impl<'a, const O: u8> DNMIEG_W<'a, O> {
    #[doc = "A cancel request is generated at a falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DNMIEG_A::_0)
    }
    #[doc = "A cancel request is generated at a rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DNMIEG_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - LVD1 Edge Select"]
    #[inline(always)]
    pub fn dlvd1ieg(&self) -> DLVD1IEG_R {
        DLVD1IEG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LVD2 Edge Select"]
    #[inline(always)]
    pub fn dlvd2ieg(&self) -> DLVD2IEG_R {
        DLVD2IEG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Pin Edge Select"]
    #[inline(always)]
    pub fn dnmieg(&self) -> DNMIEG_R {
        DNMIEG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LVD1 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dlvd1ieg(&mut self) -> DLVD1IEG_W<0> {
        DLVD1IEG_W::new(self)
    }
    #[doc = "Bit 1 - LVD2 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dlvd2ieg(&mut self) -> DLVD2IEG_W<1> {
        DLVD2IEG_W::new(self)
    }
    #[doc = "Bit 4 - NMI Pin Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn dnmieg(&mut self) -> DNMIEG_W<4> {
        DNMIEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Edge Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsiegr2](index.html) module"]
pub struct DPSIEGR2_SPEC;
impl crate::RegisterSpec for DPSIEGR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsiegr2::R](R) reader structure"]
impl crate::Readable for DPSIEGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsiegr2::W](W) writer structure"]
impl crate::Writable for DPSIEGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIEGR2 to value 0"]
impl crate::Resettable for DPSIEGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
