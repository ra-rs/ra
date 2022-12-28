#[doc = "Register `SD_STOP` reader"]
pub struct R(crate::R<SD_STOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_STOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_STOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_STOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_STOP` writer"]
pub struct W(crate::W<SD_STOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_STOP_SPEC>;
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
impl From<crate::W<SD_STOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_STOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STP` reader - Transfer Stop"]
pub type STP_R = crate::BitReader<bool>;
#[doc = "Field `STP` writer - Transfer Stop"]
pub type STP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_STOP_SPEC, bool, O>;
#[doc = "Field `SEC` reader - Block Count Register Value Select"]
pub type SEC_R = crate::BitReader<SEC_A>;
#[doc = "Block Count Register Value Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_A {
    #[doc = "0: Disable SD_SECCNT register value"]
    _0 = 0,
    #[doc = "1: Enable SD_SECCNT register value"]
    _1 = 1,
}
impl From<SEC_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_A {
        match self.bits {
            false => SEC_A::_0,
            true => SEC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEC_A::_1
    }
}
#[doc = "Field `SEC` writer - Block Count Register Value Select"]
pub type SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_STOP_SPEC, SEC_A, O>;
impl<'a, const O: u8> SEC_W<'a, O> {
    #[doc = "Disable SD_SECCNT register value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEC_A::_0)
    }
    #[doc = "Enable SD_SECCNT register value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Stop"]
    #[inline(always)]
    pub fn stp(&self) -> STP_R {
        STP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Block Count Register Value Select"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stp(&mut self) -> STP_W<0> {
        STP_W::new(self)
    }
    #[doc = "Bit 8 - Block Count Register Value Select"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<8> {
        SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Stop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_stop](index.html) module"]
pub struct SD_STOP_SPEC;
impl crate::RegisterSpec for SD_STOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_stop::R](R) reader structure"]
impl crate::Readable for SD_STOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_stop::W](W) writer structure"]
impl crate::Writable for SD_STOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_STOP to value 0"]
impl crate::Resettable for SD_STOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
