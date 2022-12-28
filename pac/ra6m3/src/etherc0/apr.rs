#[doc = "Register `APR` reader"]
pub struct R(crate::R<APR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APR` writer"]
pub struct W(crate::W<APR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APR_SPEC>;
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
impl From<crate::W<APR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP` reader - Automatic PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is automatically transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed."]
pub type AP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AP` writer - Automatic PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is automatically transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed."]
pub type AP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Automatic PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is automatically transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed."]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Automatic PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is automatically transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed."]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> AP_W<0> {
        AP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automatic PAUSE Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apr](index.html) module"]
pub struct APR_SPEC;
impl crate::RegisterSpec for APR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apr::R](R) reader structure"]
impl crate::Readable for APR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apr::W](W) writer structure"]
impl crate::Writable for APR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APR to value 0"]
impl crate::Resettable for APR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
