#[doc = "Register `DAPC` reader"]
pub struct R(crate::R<DAPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAPC` writer"]
pub struct W(crate::W<DAPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAPC_SPEC>;
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
impl From<crate::W<DAPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUMPEN` reader - Charge Pump Enable"]
pub type PUMPEN_R = crate::BitReader<PUMPEN_A>;
#[doc = "Charge Pump Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUMPEN_A {
    #[doc = "0: Charge pump disabled"]
    _0 = 0,
    #[doc = "1: Charge pump enabled"]
    _1 = 1,
}
impl From<PUMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PUMPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PUMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUMPEN_A {
        match self.bits {
            false => PUMPEN_A::_0,
            true => PUMPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PUMPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PUMPEN_A::_1
    }
}
#[doc = "Field `PUMPEN` writer - Charge Pump Enable"]
pub type PUMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAPC_SPEC, PUMPEN_A, O>;
impl<'a, const O: u8> PUMPEN_W<'a, O> {
    #[doc = "Charge pump disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PUMPEN_A::_0)
    }
    #[doc = "Charge pump enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PUMPEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Charge Pump Enable"]
    #[inline(always)]
    pub fn pumpen(&self) -> PUMPEN_R {
        PUMPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Charge Pump Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pumpen(&mut self) -> PUMPEN_W<0> {
        PUMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A Switch Charge Pump Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dapc](index.html) module"]
pub struct DAPC_SPEC;
impl crate::RegisterSpec for DAPC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dapc::R](R) reader structure"]
impl crate::Readable for DAPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dapc::W](W) writer structure"]
impl crate::Writable for DAPC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAPC to value 0"]
impl crate::Resettable for DAPC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
