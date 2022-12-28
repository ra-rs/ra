#[doc = "Register `SYPNUMR` reader"]
pub struct R(crate::R<SYPNUMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYPNUMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYPNUMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYPNUMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYPNUMR` writer"]
pub struct W(crate::W<SYPNUMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYPNUMR_SPEC>;
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
impl From<crate::W<SYPNUMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYPNUMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PNUM` reader - Local Port Number SettingThese bits hold the setting for the port number of the local port."]
pub type PNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PNUM` writer - Local Port Number SettingThese bits hold the setting for the port number of the local port."]
pub type PNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYPNUMR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Local Port Number SettingThese bits hold the setting for the port number of the local port."]
    #[inline(always)]
    pub fn pnum(&self) -> PNUM_R {
        PNUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Local Port Number SettingThese bits hold the setting for the port number of the local port."]
    #[inline(always)]
    #[must_use]
    pub fn pnum(&mut self) -> PNUM_W<0> {
        PNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Local Port Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sypnumr](index.html) module"]
pub struct SYPNUMR_SPEC;
impl crate::RegisterSpec for SYPNUMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sypnumr::R](R) reader structure"]
impl crate::Readable for SYPNUMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sypnumr::W](W) writer structure"]
impl crate::Writable for SYPNUMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYPNUMR to value 0"]
impl crate::Resettable for SYPNUMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
