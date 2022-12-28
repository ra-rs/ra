#[doc = "Register `MOSCWTCR` reader"]
pub struct R(crate::R<MOSCWTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSCWTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSCWTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSCWTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOSCWTCR` writer"]
pub struct W(crate::W<MOSCWTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOSCWTCR_SPEC>;
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
impl From<crate::W<MOSCWTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOSCWTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTS` reader - Main Clock Oscillator Wait Time Setting"]
pub type MSTS_R = crate::FieldReader<u8, MSTS_A>;
#[doc = "Main Clock Oscillator Wait Time Setting\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSTS_A {
    #[doc = "0: Wait time = 2 cycles (0.25 us)"]
    _0X0 = 0,
    #[doc = "1: Wait time = 1024 cycles (128 us)"]
    _0X1 = 1,
    #[doc = "2: Wait time = 2048 cycles (256 us)"]
    _0X2 = 2,
    #[doc = "3: Wait time = 4096 cycles (512 us)"]
    _0X3 = 3,
    #[doc = "4: Wait time = 8192 cycles (1024 us)"]
    _0X4 = 4,
    #[doc = "5: Wait time = 16384 cycles (2048 us)"]
    _0X5 = 5,
    #[doc = "6: Wait time = 32768 cycles (4096 us)"]
    _0X6 = 6,
    #[doc = "7: Wait time = 65536 cycles (8192 us)"]
    _0X7 = 7,
    #[doc = "8: Wait time = 131072 cycles (16384 us)"]
    _0X8 = 8,
    #[doc = "9: Wait time = 262144 cycles (32768 us)"]
    _0X9 = 9,
}
impl From<MSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTS_A) -> Self {
        variant as _
    }
}
impl MSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSTS_A> {
        match self.bits {
            0 => Some(MSTS_A::_0X0),
            1 => Some(MSTS_A::_0X1),
            2 => Some(MSTS_A::_0X2),
            3 => Some(MSTS_A::_0X3),
            4 => Some(MSTS_A::_0X4),
            5 => Some(MSTS_A::_0X5),
            6 => Some(MSTS_A::_0X6),
            7 => Some(MSTS_A::_0X7),
            8 => Some(MSTS_A::_0X8),
            9 => Some(MSTS_A::_0X9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == MSTS_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == MSTS_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == MSTS_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == MSTS_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == MSTS_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == MSTS_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == MSTS_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == MSTS_A::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == MSTS_A::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == MSTS_A::_0X9
    }
}
#[doc = "Field `MSTS` writer - Main Clock Oscillator Wait Time Setting"]
pub type MSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MOSCWTCR_SPEC, u8, MSTS_A, 4, O>;
impl<'a, const O: u8> MSTS_W<'a, O> {
    #[doc = "Wait time = 2 cycles (0.25 us)"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(MSTS_A::_0X0)
    }
    #[doc = "Wait time = 1024 cycles (128 us)"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(MSTS_A::_0X1)
    }
    #[doc = "Wait time = 2048 cycles (256 us)"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(MSTS_A::_0X2)
    }
    #[doc = "Wait time = 4096 cycles (512 us)"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(MSTS_A::_0X3)
    }
    #[doc = "Wait time = 8192 cycles (1024 us)"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(MSTS_A::_0X4)
    }
    #[doc = "Wait time = 16384 cycles (2048 us)"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(MSTS_A::_0X5)
    }
    #[doc = "Wait time = 32768 cycles (4096 us)"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(MSTS_A::_0X6)
    }
    #[doc = "Wait time = 65536 cycles (8192 us)"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(MSTS_A::_0X7)
    }
    #[doc = "Wait time = 131072 cycles (16384 us)"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(MSTS_A::_0X8)
    }
    #[doc = "Wait time = 262144 cycles (32768 us)"]
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(MSTS_A::_0X9)
    }
}
impl R {
    #[doc = "Bits 0:3 - Main Clock Oscillator Wait Time Setting"]
    #[inline(always)]
    pub fn msts(&self) -> MSTS_R {
        MSTS_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Main Clock Oscillator Wait Time Setting"]
    #[inline(always)]
    #[must_use]
    pub fn msts(&mut self) -> MSTS_W<0> {
        MSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Oscillator Wait Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moscwtcr](index.html) module"]
pub struct MOSCWTCR_SPEC;
impl crate::RegisterSpec for MOSCWTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [moscwtcr::R](R) reader structure"]
impl crate::Readable for MOSCWTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [moscwtcr::W](W) writer structure"]
impl crate::Writable for MOSCWTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOSCWTCR to value 0x05"]
impl crate::Resettable for MOSCWTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
