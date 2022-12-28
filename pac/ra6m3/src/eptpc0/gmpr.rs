#[doc = "Register `GMPR` reader"]
pub struct R(crate::R<GMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMPR` writer"]
pub struct W(crate::W<GMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMPR_SPEC>;
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
impl From<crate::W<GMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GMPR2` reader - grandmasterPriority2 Field Value SettingThese bits are used to set the value of the grandmasterPriority2 fields of Announce messages."]
pub type GMPR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GMPR2` writer - grandmasterPriority2 Field Value SettingThese bits are used to set the value of the grandmasterPriority2 fields of Announce messages."]
pub type GMPR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GMPR_SPEC, u8, u8, 8, O>;
#[doc = "Field `GMPR1` reader - grandmasterPriority1 Field Value SettingThese bits are used to set the value of the grandmasterPriority1 fields of Announce messages."]
pub type GMPR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GMPR1` writer - grandmasterPriority1 Field Value SettingThese bits are used to set the value of the grandmasterPriority1 fields of Announce messages."]
pub type GMPR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GMPR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - grandmasterPriority2 Field Value SettingThese bits are used to set the value of the grandmasterPriority2 fields of Announce messages."]
    #[inline(always)]
    pub fn gmpr2(&self) -> GMPR2_R {
        GMPR2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - grandmasterPriority1 Field Value SettingThese bits are used to set the value of the grandmasterPriority1 fields of Announce messages."]
    #[inline(always)]
    pub fn gmpr1(&self) -> GMPR1_R {
        GMPR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - grandmasterPriority2 Field Value SettingThese bits are used to set the value of the grandmasterPriority2 fields of Announce messages."]
    #[inline(always)]
    #[must_use]
    pub fn gmpr2(&mut self) -> GMPR2_W<0> {
        GMPR2_W::new(self)
    }
    #[doc = "Bits 16:23 - grandmasterPriority1 Field Value SettingThese bits are used to set the value of the grandmasterPriority1 fields of Announce messages."]
    #[inline(always)]
    #[must_use]
    pub fn gmpr1(&mut self) -> GMPR1_W<16> {
        GMPR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "grandmasterPriority Field Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmpr](index.html) module"]
pub struct GMPR_SPEC;
impl crate::RegisterSpec for GMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmpr::R](R) reader structure"]
impl crate::Readable for GMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmpr::W](W) writer structure"]
impl crate::Writable for GMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GMPR to value 0"]
impl crate::Resettable for GMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
