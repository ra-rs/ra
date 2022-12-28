#[doc = "Register `MPR` writer"]
pub struct W(crate::W<MPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPR_SPEC>;
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
impl From<crate::W<MPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MP` writer - Manual PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is manually transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed. The read value is undefined."]
pub type MP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPR_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Manual PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is manually transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed. The read value is undefined."]
    #[inline(always)]
    #[must_use]
    pub fn mp(&mut self) -> MP_W<0> {
        MP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manual PAUSE Frame Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpr](index.html) module"]
pub struct MPR_SPEC;
impl crate::RegisterSpec for MPR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mpr::W](W) writer structure"]
impl crate::Writable for MPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPR to value 0"]
impl crate::Resettable for MPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
