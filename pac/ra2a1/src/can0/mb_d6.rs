#[doc = "Register `MB%s_D6` reader"]
pub struct R(crate::R<MB_D6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MB_D6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MB_D6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MB_D6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MB%s_D6` writer"]
pub struct W(crate::W<MB_D6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MB_D6_SPEC>;
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
impl From<crate::W<MB_D6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MB_D6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA6` reader - Data Bytes 6DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type DATA6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA6` writer - Data Bytes 6DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
pub type DATA6_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MB_D6_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data Bytes 6DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Bytes 6DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<0> {
        DATA6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mb_d6](index.html) module"]
pub struct MB_D6_SPEC;
impl crate::RegisterSpec for MB_D6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mb_d6::R](R) reader structure"]
impl crate::Readable for MB_D6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mb_d6::W](W) writer structure"]
impl crate::Writable for MB_D6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MB%s_D6 to value 0"]
impl crate::Resettable for MB_D6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
