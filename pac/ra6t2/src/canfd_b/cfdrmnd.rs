#[doc = "Register `CFDRMND` reader"]
pub struct R(crate::R<CFDRMND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRMND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRMND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRMND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDRMND` writer"]
pub struct W(crate::W<CFDRMND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDRMND_SPEC>;
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
impl From<crate::W<CFDRMND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDRMND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMNS` reader - RX Message Buffer New Data Status"]
pub type RMNS_R = crate::FieldReader<u32, RMNS_A>;
#[doc = "RX Message Buffer New Data Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RMNS_A {
    #[doc = "0: New data not stored in corresponding RX message buffer"]
    _0 = 0,
    #[doc = "1: New data stored in corresponding RX message buffer"]
    _1 = 1,
}
impl From<RMNS_A> for u32 {
    #[inline(always)]
    fn from(variant: RMNS_A) -> Self {
        variant as _
    }
}
impl RMNS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RMNS_A> {
        match self.bits {
            0 => Some(RMNS_A::_0),
            1 => Some(RMNS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMNS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMNS_A::_1
    }
}
#[doc = "Field `RMNS` writer - RX Message Buffer New Data Status"]
pub type RMNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDRMND_SPEC, u32, RMNS_A, 32, O>;
impl<'a, const O: u8> RMNS_W<'a, O> {
    #[doc = "New data not stored in corresponding RX message buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMNS_A::_0)
    }
    #[doc = "New data stored in corresponding RX message buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMNS_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - RX Message Buffer New Data Status"]
    #[inline(always)]
    pub fn rmns(&self) -> RMNS_R {
        RMNS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RX Message Buffer New Data Status"]
    #[inline(always)]
    #[must_use]
    pub fn rmns(&mut self) -> RMNS_W<0> {
        RMNS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Message Buffer New Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrmnd](index.html) module"]
pub struct CFDRMND_SPEC;
impl crate::RegisterSpec for CFDRMND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrmnd::R](R) reader structure"]
impl crate::Readable for CFDRMND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdrmnd::W](W) writer structure"]
impl crate::Writable for CFDRMND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDRMND to value 0"]
impl crate::Resettable for CFDRMND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
