#[doc = "Register `BTZF%sERRRW` reader"]
pub struct R(crate::R<BTZFERRRW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTZFERRRW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTZFERRRW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTZFERRRW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTZF%sERRRW` writer"]
pub struct W(crate::W<BTZFERRRW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTZFERRRW_SPEC>;
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
impl From<crate::W<BTZFERRRW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTZFERRRW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRWSTAT` reader - TrustZone filter error access Read/Write Status"]
pub type TRWSTAT_R = crate::BitReader<TRWSTAT_A>;
#[doc = "TrustZone filter error access Read/Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRWSTAT_A {
    #[doc = "0: Read access"]
    _0 = 0,
    #[doc = "1: Write access"]
    _1 = 1,
}
impl From<TRWSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: TRWSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl TRWSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRWSTAT_A {
        match self.bits {
            false => TRWSTAT_A::_0,
            true => TRWSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRWSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRWSTAT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - TrustZone filter error access Read/Write Status"]
    #[inline(always)]
    pub fn trwstat(&self) -> TRWSTAT_R {
        TRWSTAT_R::new((self.bits & 1) != 0)
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
#[doc = "BUS TZF Error Read Write Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btzferrrw](index.html) module"]
pub struct BTZFERRRW_SPEC;
impl crate::RegisterSpec for BTZFERRRW_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [btzferrrw::R](R) reader structure"]
impl crate::Readable for BTZFERRRW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btzferrrw::W](W) writer structure"]
impl crate::Writable for BTZFERRRW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTZF%sERRRW to value 0"]
impl crate::Resettable for BTZFERRRW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
