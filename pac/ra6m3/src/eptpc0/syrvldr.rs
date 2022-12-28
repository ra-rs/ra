#[doc = "Register `SYRVLDR` writer"]
pub struct W(crate::W<SYRVLDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYRVLDR_SPEC>;
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
impl From<crate::W<SYRVLDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYRVLDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BMC Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMUP_AW {
    #[doc = "0: no effect"]
    _0 = 0,
    #[doc = "1: Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers holding the MasterClock identifying information."]
    _1 = 1,
}
impl From<BMUP_AW> for bool {
    #[inline(always)]
    fn from(variant: BMUP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMUP` writer - BMC Update"]
pub type BMUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRVLDR_SPEC, BMUP_AW, O>;
impl<'a, const O: u8> BMUP_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BMUP_AW::_0)
    }
    #[doc = "Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers holding the MasterClock identifying information."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BMUP_AW::_1)
    }
}
#[doc = "State Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STUP_AW {
    #[doc = "0: no effect"]
    _0 = 0,
    #[doc = "1: Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers related to the reception and transmission of PTP messages."]
    _1 = 1,
}
impl From<STUP_AW> for bool {
    #[inline(always)]
    fn from(variant: STUP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STUP` writer - State Update"]
pub type STUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRVLDR_SPEC, STUP_AW, O>;
impl<'a, const O: u8> STUP_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STUP_AW::_0)
    }
    #[doc = "Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers related to the reception and transmission of PTP messages."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STUP_AW::_1)
    }
}
#[doc = "Announce Message Generation Information Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANUP_AW {
    #[doc = "0: no effect"]
    _0 = 0,
    #[doc = "1: Setting this bit to 1 leads to simultaneous reflection in the Announce message generation block of the values of the registers required for the generation of Announce messages."]
    _1 = 1,
}
impl From<ANUP_AW> for bool {
    #[inline(always)]
    fn from(variant: ANUP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANUP` writer - Announce Message Generation Information Update"]
pub type ANUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYRVLDR_SPEC, ANUP_AW, O>;
impl<'a, const O: u8> ANUP_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANUP_AW::_0)
    }
    #[doc = "Setting this bit to 1 leads to simultaneous reflection in the Announce message generation block of the values of the registers required for the generation of Announce messages."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANUP_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - BMC Update"]
    #[inline(always)]
    #[must_use]
    pub fn bmup(&mut self) -> BMUP_W<0> {
        BMUP_W::new(self)
    }
    #[doc = "Bit 1 - State Update"]
    #[inline(always)]
    #[must_use]
    pub fn stup(&mut self) -> STUP_W<1> {
        STUP_W::new(self)
    }
    #[doc = "Bit 2 - Announce Message Generation Information Update"]
    #[inline(always)]
    #[must_use]
    pub fn anup(&mut self) -> ANUP_W<2> {
        ANUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Register Value Load Directive Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syrvldr](index.html) module"]
pub struct SYRVLDR_SPEC;
impl crate::RegisterSpec for SYRVLDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [syrvldr::W](W) writer structure"]
impl crate::Writable for SYRVLDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYRVLDR to value 0"]
impl crate::Resettable for SYRVLDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
