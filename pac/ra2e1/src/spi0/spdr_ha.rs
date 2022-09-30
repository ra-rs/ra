#[doc = "Register `SPDR_HA` reader"]
pub struct R(crate::R<SPDR_HA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDR_HA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDR_HA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDR_HA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDR_HA` writer"]
pub struct W(crate::W<SPDR_HA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDR_HA_SPEC>;
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
impl From<crate::W<SPDR_HA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDR_HA_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdr_ha](index.html) module"]
pub struct SPDR_HA_SPEC;
impl crate::RegisterSpec for SPDR_HA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spdr_ha::R](R) reader structure"]
impl crate::Readable for SPDR_HA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdr_ha::W](W) writer structure"]
impl crate::Writable for SPDR_HA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPDR_HA to value 0"]
impl crate::Resettable for SPDR_HA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
