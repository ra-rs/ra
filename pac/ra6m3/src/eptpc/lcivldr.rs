#[doc = "Register `LCIVLDR` writer"]
pub struct W(crate::W<LCIVLDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCIVLDR_SPEC>;
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
impl From<crate::W<LCIVLDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCIVLDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Local Time Counter Initial Value Load Directive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOAD_AW {
    #[doc = "0: The initial value is not loaded into the local time counter."]
    _0 = 0,
    #[doc = "1: The initial value is loaded into the local time counter."]
    _1 = 1,
}
impl From<LOAD_AW> for bool {
    #[inline(always)]
    fn from(variant: LOAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOAD` writer - Local Time Counter Initial Value Load Directive"]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCIVLDR_SPEC, LOAD_AW, O>;
impl<'a, const O: u8> LOAD_W<'a, O> {
    #[doc = "The initial value is not loaded into the local time counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOAD_AW::_0)
    }
    #[doc = "The initial value is loaded into the local time counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOAD_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Local Time Counter Initial Value Load Directive"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local Time Counter Initial Value Load Directive Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcivldr](index.html) module"]
pub struct LCIVLDR_SPEC;
impl crate::RegisterSpec for LCIVLDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lcivldr::W](W) writer structure"]
impl crate::Writable for LCIVLDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCIVLDR to value 0"]
impl crate::Resettable for LCIVLDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
