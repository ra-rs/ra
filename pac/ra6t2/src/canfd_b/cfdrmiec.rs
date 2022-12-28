#[doc = "Register `CFDRMIEC` reader"]
pub struct R(crate::R<CFDRMIEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRMIEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRMIEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRMIEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDRMIEC` writer"]
pub struct W(crate::W<CFDRMIEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDRMIEC_SPEC>;
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
impl From<crate::W<CFDRMIEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDRMIEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMIEg` reader - RX Message Buffer Interrupt Enable"]
pub type RMIEG_R = crate::FieldReader<u32, RMIEG_A>;
#[doc = "RX Message Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RMIEG_A {
    #[doc = "0: RX Message Buffer Interrupt disabled for corresponding RX message buffer"]
    _0 = 0,
    #[doc = "1: RX Message Buffer Interrupt enabled for corresponding RX message buffer"]
    _1 = 1,
}
impl From<RMIEG_A> for u32 {
    #[inline(always)]
    fn from(variant: RMIEG_A) -> Self {
        variant as _
    }
}
impl RMIEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RMIEG_A> {
        match self.bits {
            0 => Some(RMIEG_A::_0),
            1 => Some(RMIEG_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMIEG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMIEG_A::_1
    }
}
#[doc = "Field `RMIEg` writer - RX Message Buffer Interrupt Enable"]
pub type RMIEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDRMIEC_SPEC, u32, RMIEG_A, 32, O>;
impl<'a, const O: u8> RMIEG_W<'a, O> {
    #[doc = "RX Message Buffer Interrupt disabled for corresponding RX message buffer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMIEG_A::_0)
    }
    #[doc = "RX Message Buffer Interrupt enabled for corresponding RX message buffer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMIEG_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - RX Message Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn rmieg(&self) -> RMIEG_R {
        RMIEG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RX Message Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmieg(&mut self) -> RMIEG_W<0> {
        RMIEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Message Buffer Interrupt Enable Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrmiec](index.html) module"]
pub struct CFDRMIEC_SPEC;
impl crate::RegisterSpec for CFDRMIEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrmiec::R](R) reader structure"]
impl crate::Readable for CFDRMIEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdrmiec::W](W) writer structure"]
impl crate::Writable for CFDRMIEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDRMIEC to value 0"]
impl crate::Resettable for CFDRMIEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
