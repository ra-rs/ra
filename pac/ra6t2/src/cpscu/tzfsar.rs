#[doc = "Register `TZFSAR` reader"]
pub struct R(crate::R<TZFSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZFSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZFSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZFSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZFSAR` writer"]
pub struct W(crate::W<TZFSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZFSAR_SPEC>;
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
impl From<crate::W<TZFSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZFSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZFSA0` reader - Security attributes of registers for TrustZone Filter"]
pub type TZFSA0_R = crate::BitReader<TZFSA0_A>;
#[doc = "Security attributes of registers for TrustZone Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TZFSA0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<TZFSA0_A> for bool {
    #[inline(always)]
    fn from(variant: TZFSA0_A) -> Self {
        variant as u8 != 0
    }
}
impl TZFSA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZFSA0_A {
        match self.bits {
            false => TZFSA0_A::_0,
            true => TZFSA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TZFSA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TZFSA0_A::_1
    }
}
#[doc = "Field `TZFSA0` writer - Security attributes of registers for TrustZone Filter"]
pub type TZFSA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZFSAR_SPEC, TZFSA0_A, O>;
impl<'a, const O: u8> TZFSA0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TZFSA0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TZFSA0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for TrustZone Filter"]
    #[inline(always)]
    pub fn tzfsa0(&self) -> TZFSA0_R {
        TZFSA0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for TrustZone Filter"]
    #[inline(always)]
    #[must_use]
    pub fn tzfsa0(&mut self) -> TZFSA0_W<0> {
        TZFSA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TrustZone Filter Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzfsar](index.html) module"]
pub struct TZFSAR_SPEC;
impl crate::RegisterSpec for TZFSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzfsar::R](R) reader structure"]
impl crate::Readable for TZFSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzfsar::W](W) writer structure"]
impl crate::Writable for TZFSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TZFSAR to value 0xffff_fffe"]
impl crate::Resettable for TZFSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_fffe;
}
