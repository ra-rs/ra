#[doc = "Register `ADSTOPR` writer"]
pub struct W(crate::W<ADSTOPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSTOPR_SPEC>;
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
impl From<crate::W<ADSTOPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSTOPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D Converter Unit 0 Force Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTOP0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Force stop the operation of A/D converter unit 0"]
    _1 = 1,
}
impl From<ADSTOP0_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSTOP0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTOP0` writer - A/D Converter Unit 0 Force Stop"]
pub type ADSTOP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSTOPR_SPEC, ADSTOP0_AW, O>;
impl<'a, const O: u8> ADSTOP0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSTOP0_AW::_0)
    }
    #[doc = "Force stop the operation of A/D converter unit 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSTOP0_AW::_1)
    }
}
#[doc = "A/D Converter Unit 1 Force Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTOP1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Force stop the operation of A/D converter unit 1"]
    _1 = 1,
}
impl From<ADSTOP1_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSTOP1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTOP1` writer - A/D Converter Unit 1 Force Stop"]
pub type ADSTOP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADSTOPR_SPEC, ADSTOP1_AW, O>;
impl<'a, const O: u8> ADSTOP1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSTOP1_AW::_0)
    }
    #[doc = "Force stop the operation of A/D converter unit 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSTOP1_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Converter Unit 0 Force Stop"]
    #[inline(always)]
    #[must_use]
    pub fn adstop0(&mut self) -> ADSTOP0_W<0> {
        ADSTOP0_W::new(self)
    }
    #[doc = "Bit 8 - A/D Converter Unit 1 Force Stop"]
    #[inline(always)]
    #[must_use]
    pub fn adstop1(&mut self) -> ADSTOP1_W<8> {
        ADSTOP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Stop Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adstopr](index.html) module"]
pub struct ADSTOPR_SPEC;
impl crate::RegisterSpec for ADSTOPR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adstopr::W](W) writer structure"]
impl crate::Writable for ADSTOPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSTOPR to value 0"]
impl crate::Resettable for ADSTOPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
