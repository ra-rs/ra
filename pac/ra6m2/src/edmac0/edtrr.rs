#[doc = "Register `EDTRR` reader"]
pub struct R(crate::R<EDTRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDTRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDTRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDTRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDTRR` writer"]
pub struct W(crate::W<EDTRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDTRR_SPEC>;
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
impl From<crate::W<EDTRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDTRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR_AW {
    #[doc = "0: no effect."]
    _0 = 0,
    #[doc = "1: When 1 is written, the EDMAC reads the corresponding descriptor and transmits frames where the TD0.TACT bit is 1. The TR bit becomes 0 after all the valid frames are transmitted."]
    _1 = 1,
}
impl From<TR_AW> for bool {
    #[inline(always)]
    fn from(variant: TR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR` writer - Transmit Request"]
pub type TR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDTRR_SPEC, TR_AW, O>;
impl<'a, const O: u8> TR_W<'a, O> {
    #[doc = "no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TR_AW::_0)
    }
    #[doc = "When 1 is written, the EDMAC reads the corresponding descriptor and transmits frames where the TD0.TACT bit is 1. The TR bit becomes 0 after all the valid frames are transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TR_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Request"]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TR_W<0> {
        TR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMAC Transmit Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edtrr](index.html) module"]
pub struct EDTRR_SPEC;
impl crate::RegisterSpec for EDTRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edtrr::R](R) reader structure"]
impl crate::Readable for EDTRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edtrr::W](W) writer structure"]
impl crate::Writable for EDTRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDTRR to value 0"]
impl crate::Resettable for EDTRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
