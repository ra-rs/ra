#[doc = "Register `ADSTR%s` writer"]
pub struct W(crate::W<ADSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSTR_SPEC>;
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
impl From<crate::W<ADSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Scan Group n A/D Conversion Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADST_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Start the A/D conversion of scan group n"]
    _1 = 1,
}
impl From<ADST_AW> for bool {
    #[inline(always)]
    fn from(variant: ADST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADST` writer - Scan Group n A/D Conversion Start"]
pub type ADST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSTR_SPEC, ADST_AW, O>;
impl<'a, const O: u8> ADST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADST_AW::_0)
    }
    #[doc = "Start the A/D conversion of scan group n"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADST_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n A/D Conversion Start"]
    #[inline(always)]
    #[must_use]
    pub fn adst(&mut self) -> ADST_W<0> {
        ADST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Software Start Register %s\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adstr](index.html) module"]
pub struct ADSTR_SPEC;
impl crate::RegisterSpec for ADSTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adstr::W](W) writer structure"]
impl crate::Writable for ADSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSTR%s to value 0"]
impl crate::Resettable for ADSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
