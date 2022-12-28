#[doc = "Register `PIPEMAXP` reader"]
pub struct R(crate::R<PIPEMAXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIPEMAXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIPEMAXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIPEMAXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIPEMAXP` writer"]
pub struct W(crate::W<PIPEMAXP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIPEMAXP_SPEC>;
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
impl From<crate::W<PIPEMAXP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIPEMAXP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MXPS` reader - Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9."]
pub type MXPS_R = crate::FieldReader<u16, MXPS_A>;
#[doc = "Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MXPS_A(u16);
impl From<MXPS_A> for u16 {
    #[inline(always)]
    fn from(val: MXPS_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `MXPS` writer - Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9."]
pub type MXPS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PIPEMAXP_SPEC, u16, MXPS_A, 11, O>;
#[doc = "Field `DEVSEL` reader - Device SelectThese bits specify the address of the peripheral device when the host controller function is selected."]
pub type DEVSEL_R = crate::FieldReader<u8, DEVSEL_A>;
#[doc = "Device SelectThese bits specify the address of the peripheral device when the host controller function is selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DEVSEL_A(u8);
impl From<DEVSEL_A> for u8 {
    #[inline(always)]
    fn from(val: DEVSEL_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `DEVSEL` writer - Device SelectThese bits specify the address of the peripheral device when the host controller function is selected."]
pub type DEVSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PIPEMAXP_SPEC, u8, DEVSEL_A, 4, O>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9."]
    #[inline(always)]
    pub fn mxps(&self) -> MXPS_R {
        MXPS_R::new(self.bits & 0x07ff)
    }
    #[doc = "Bits 12:15 - Device SelectThese bits specify the address of the peripheral device when the host controller function is selected."]
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9."]
    #[inline(always)]
    #[must_use]
    pub fn mxps(&mut self) -> MXPS_W<0> {
        MXPS_W::new(self)
    }
    #[doc = "Bits 12:15 - Device SelectThese bits specify the address of the peripheral device when the host controller function is selected."]
    #[inline(always)]
    #[must_use]
    pub fn devsel(&mut self) -> DEVSEL_W<12> {
        DEVSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Maximum Packet Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pipemaxp](index.html) module"]
pub struct PIPEMAXP_SPEC;
impl crate::RegisterSpec for PIPEMAXP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pipemaxp::R](R) reader structure"]
impl crate::Readable for PIPEMAXP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pipemaxp::W](W) writer structure"]
impl crate::Writable for PIPEMAXP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPEMAXP to value 0"]
impl crate::Resettable for PIPEMAXP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
