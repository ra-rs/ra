#[doc = "Register `BUS%sERRRW` reader"]
pub struct R(crate::R<BUSERRRW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSERRRW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSERRRW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSERRRW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS%sERRRW` writer"]
pub struct W(crate::W<BUSERRRW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSERRRW_SPEC>;
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
impl From<crate::W<BUSERRRW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSERRRW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWSTAT` reader - Error Access Read/Write Status"]
pub type RWSTAT_R = crate::BitReader<RWSTAT_A>;
#[doc = "Error Access Read/Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWSTAT_A {
    #[doc = "0: Read access"]
    _0 = 0,
    #[doc = "1: Write access"]
    _1 = 1,
}
impl From<RWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: RWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl RWSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWSTAT_A {
        match self.bits {
            false => RWSTAT_A::_0,
            true => RWSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWSTAT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Error Access Read/Write Status"]
    #[inline(always)]
    pub fn rwstat(&self) -> RWSTAT_R {
        RWSTAT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BUS Error Read Write Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buserrrw](index.html) module"]
pub struct BUSERRRW_SPEC;
impl crate::RegisterSpec for BUSERRRW_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [buserrrw::R](R) reader structure"]
impl crate::Readable for BUSERRRW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buserrrw::W](W) writer structure"]
impl crate::Writable for BUSERRRW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS%sERRRW to value 0"]
impl crate::Resettable for BUSERRRW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
