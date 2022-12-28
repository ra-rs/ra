#[doc = "Register `CFDRMNB` reader"]
pub struct R(crate::R<CFDRMNB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRMNB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRMNB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRMNB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDRMNB` writer"]
pub struct W(crate::W<CFDRMNB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDRMNB_SPEC>;
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
impl From<crate::W<CFDRMNB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDRMNB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRXMB` reader - Number of RX Message Buffers"]
pub type NRXMB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NRXMB` writer - Number of RX Message Buffers"]
pub type NRXMB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDRMNB_SPEC, u8, u8, 6, O>;
#[doc = "Field `RMPLS` reader - Reception Message Buffer Payload Data Size"]
pub type RMPLS_R = crate::FieldReader<u8, RMPLS_A>;
#[doc = "Reception Message Buffer Payload Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RMPLS_A {
    #[doc = "0: 8 bytes"]
    _000 = 0,
    #[doc = "1: 12 bytes"]
    _001 = 1,
    #[doc = "2: 16 bytes"]
    _010 = 2,
    #[doc = "3: 20 bytes"]
    _011 = 3,
    #[doc = "4: 24 bytes"]
    _100 = 4,
    #[doc = "5: 32 bytes"]
    _101 = 5,
    #[doc = "6: 48 bytes"]
    _110 = 6,
    #[doc = "7: 64 bytes"]
    _111 = 7,
}
impl From<RMPLS_A> for u8 {
    #[inline(always)]
    fn from(variant: RMPLS_A) -> Self {
        variant as _
    }
}
impl RMPLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMPLS_A {
        match self.bits {
            0 => RMPLS_A::_000,
            1 => RMPLS_A::_001,
            2 => RMPLS_A::_010,
            3 => RMPLS_A::_011,
            4 => RMPLS_A::_100,
            5 => RMPLS_A::_101,
            6 => RMPLS_A::_110,
            7 => RMPLS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RMPLS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RMPLS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RMPLS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RMPLS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RMPLS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RMPLS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RMPLS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RMPLS_A::_111
    }
}
#[doc = "Field `RMPLS` writer - Reception Message Buffer Payload Data Size"]
pub type RMPLS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFDRMNB_SPEC, u8, RMPLS_A, 3, O>;
impl<'a, const O: u8> RMPLS_W<'a, O> {
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RMPLS_A::_000)
    }
    #[doc = "12 bytes"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RMPLS_A::_001)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RMPLS_A::_010)
    }
    #[doc = "20 bytes"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RMPLS_A::_011)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RMPLS_A::_100)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RMPLS_A::_101)
    }
    #[doc = "48 bytes"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RMPLS_A::_110)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RMPLS_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:5 - Number of RX Message Buffers"]
    #[inline(always)]
    pub fn nrxmb(&self) -> NRXMB_R {
        NRXMB_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - Reception Message Buffer Payload Data Size"]
    #[inline(always)]
    pub fn rmpls(&self) -> RMPLS_R {
        RMPLS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of RX Message Buffers"]
    #[inline(always)]
    #[must_use]
    pub fn nrxmb(&mut self) -> NRXMB_W<0> {
        NRXMB_W::new(self)
    }
    #[doc = "Bits 8:10 - Reception Message Buffer Payload Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn rmpls(&mut self) -> RMPLS_W<8> {
        RMPLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Message Buffer Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrmnb](index.html) module"]
pub struct CFDRMNB_SPEC;
impl crate::RegisterSpec for CFDRMNB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrmnb::R](R) reader structure"]
impl crate::Readable for CFDRMNB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdrmnb::W](W) writer structure"]
impl crate::Writable for CFDRMNB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDRMNB to value 0"]
impl crate::Resettable for CFDRMNB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
