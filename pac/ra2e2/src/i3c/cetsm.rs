#[doc = "Register `CETSM` reader"]
pub struct R(crate::R<CETSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CETSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CETSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CETSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CETSM` writer"]
pub struct W(crate::W<CETSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CETSM_SPEC>;
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
impl From<crate::W<CETSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CETSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQ` reader - Frequency Byte"]
pub type FREQ_R = crate::FieldReader<u8, FREQ_A>;
#[doc = "Frequency Byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FREQ_A {
    #[doc = "0: 32.0 KHz"]
    _0X00 = 0,
    #[doc = "15: 7.5 MHz"]
    _0X0F = 15,
    #[doc = "31: 15.5 MHz"]
    _0X1F = 31,
    #[doc = "47: 23.5 MHz"]
    _0X2F = 47,
    #[doc = "63: 31.5 MHz"]
    _0X3F = 63,
    #[doc = "79: 39.5 MHz"]
    _0X4F = 79,
    #[doc = "95: 47.5 MHz"]
    _0X5F = 95,
    #[doc = "111: 55.5 MHz"]
    _0X6F = 111,
    #[doc = "127: 63.5 MHz"]
    _0X7F = 127,
}
impl From<FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as _
    }
}
impl FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQ_A> {
        match self.bits {
            0 => Some(FREQ_A::_0X00),
            15 => Some(FREQ_A::_0X0F),
            31 => Some(FREQ_A::_0X1F),
            47 => Some(FREQ_A::_0X2F),
            63 => Some(FREQ_A::_0X3F),
            79 => Some(FREQ_A::_0X4F),
            95 => Some(FREQ_A::_0X5F),
            111 => Some(FREQ_A::_0X6F),
            127 => Some(FREQ_A::_0X7F),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == FREQ_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == FREQ_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X1F`"]
    #[inline(always)]
    pub fn is_0x1f(&self) -> bool {
        *self == FREQ_A::_0X1F
    }
    #[doc = "Checks if the value of the field is `_0X2F`"]
    #[inline(always)]
    pub fn is_0x2f(&self) -> bool {
        *self == FREQ_A::_0X2F
    }
    #[doc = "Checks if the value of the field is `_0X3F`"]
    #[inline(always)]
    pub fn is_0x3f(&self) -> bool {
        *self == FREQ_A::_0X3F
    }
    #[doc = "Checks if the value of the field is `_0X4F`"]
    #[inline(always)]
    pub fn is_0x4f(&self) -> bool {
        *self == FREQ_A::_0X4F
    }
    #[doc = "Checks if the value of the field is `_0X5F`"]
    #[inline(always)]
    pub fn is_0x5f(&self) -> bool {
        *self == FREQ_A::_0X5F
    }
    #[doc = "Checks if the value of the field is `_0X6F`"]
    #[inline(always)]
    pub fn is_0x6f(&self) -> bool {
        *self == FREQ_A::_0X6F
    }
    #[doc = "Checks if the value of the field is `_0X7F`"]
    #[inline(always)]
    pub fn is_0x7f(&self) -> bool {
        *self == FREQ_A::_0X7F
    }
}
#[doc = "Field `FREQ` writer - Frequency Byte"]
pub type FREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CETSM_SPEC, u8, FREQ_A, 8, O>;
impl<'a, const O: u8> FREQ_W<'a, O> {
    #[doc = "32.0 KHz"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(FREQ_A::_0X00)
    }
    #[doc = "7.5 MHz"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(FREQ_A::_0X0F)
    }
    #[doc = "15.5 MHz"]
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut W {
        self.variant(FREQ_A::_0X1F)
    }
    #[doc = "23.5 MHz"]
    #[inline(always)]
    pub fn _0x2f(self) -> &'a mut W {
        self.variant(FREQ_A::_0X2F)
    }
    #[doc = "31.5 MHz"]
    #[inline(always)]
    pub fn _0x3f(self) -> &'a mut W {
        self.variant(FREQ_A::_0X3F)
    }
    #[doc = "39.5 MHz"]
    #[inline(always)]
    pub fn _0x4f(self) -> &'a mut W {
        self.variant(FREQ_A::_0X4F)
    }
    #[doc = "47.5 MHz"]
    #[inline(always)]
    pub fn _0x5f(self) -> &'a mut W {
        self.variant(FREQ_A::_0X5F)
    }
    #[doc = "55.5 MHz"]
    #[inline(always)]
    pub fn _0x6f(self) -> &'a mut W {
        self.variant(FREQ_A::_0X6F)
    }
    #[doc = "63.5 MHz"]
    #[inline(always)]
    pub fn _0x7f(self) -> &'a mut W {
        self.variant(FREQ_A::_0X7F)
    }
}
#[doc = "Field `INAC` reader - Inaccuracy Byte"]
pub type INAC_R = crate::FieldReader<u8, INAC_A>;
#[doc = "Inaccuracy Byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INAC_A {
    #[doc = "0: 0.0%"]
    _0X00 = 0,
    #[doc = "15: 1.5%"]
    _0X0F = 15,
    #[doc = "31: 3.1%"]
    _0X1F = 31,
    #[doc = "47: 4.7%"]
    _0X2F = 47,
    #[doc = "63: 6.3%"]
    _0X3F = 63,
    #[doc = "79: 7.9%"]
    _0X4F = 79,
    #[doc = "95: 9.5%"]
    _0X5F = 95,
    #[doc = "111: 11.1%"]
    _0X6F = 111,
    #[doc = "127: 12.7%"]
    _0X7F = 127,
    #[doc = "143: 14.3%"]
    _0X8F = 143,
    #[doc = "159: 15.9%"]
    _0X9F = 159,
    #[doc = "175: 17.5%"]
    _0X_AF = 175,
    #[doc = "191: 19.1%"]
    _0X_BF = 191,
    #[doc = "207: 20.7%"]
    _0X_CF = 207,
    #[doc = "223: 22.3%"]
    _0X_DF = 223,
    #[doc = "239: 23.9%"]
    _0X_EF = 239,
    #[doc = "255: 25.5%"]
    _0X_FF = 255,
}
impl From<INAC_A> for u8 {
    #[inline(always)]
    fn from(variant: INAC_A) -> Self {
        variant as _
    }
}
impl INAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INAC_A> {
        match self.bits {
            0 => Some(INAC_A::_0X00),
            15 => Some(INAC_A::_0X0F),
            31 => Some(INAC_A::_0X1F),
            47 => Some(INAC_A::_0X2F),
            63 => Some(INAC_A::_0X3F),
            79 => Some(INAC_A::_0X4F),
            95 => Some(INAC_A::_0X5F),
            111 => Some(INAC_A::_0X6F),
            127 => Some(INAC_A::_0X7F),
            143 => Some(INAC_A::_0X8F),
            159 => Some(INAC_A::_0X9F),
            175 => Some(INAC_A::_0X_AF),
            191 => Some(INAC_A::_0X_BF),
            207 => Some(INAC_A::_0X_CF),
            223 => Some(INAC_A::_0X_DF),
            239 => Some(INAC_A::_0X_EF),
            255 => Some(INAC_A::_0X_FF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == INAC_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == INAC_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X1F`"]
    #[inline(always)]
    pub fn is_0x1f(&self) -> bool {
        *self == INAC_A::_0X1F
    }
    #[doc = "Checks if the value of the field is `_0X2F`"]
    #[inline(always)]
    pub fn is_0x2f(&self) -> bool {
        *self == INAC_A::_0X2F
    }
    #[doc = "Checks if the value of the field is `_0X3F`"]
    #[inline(always)]
    pub fn is_0x3f(&self) -> bool {
        *self == INAC_A::_0X3F
    }
    #[doc = "Checks if the value of the field is `_0X4F`"]
    #[inline(always)]
    pub fn is_0x4f(&self) -> bool {
        *self == INAC_A::_0X4F
    }
    #[doc = "Checks if the value of the field is `_0X5F`"]
    #[inline(always)]
    pub fn is_0x5f(&self) -> bool {
        *self == INAC_A::_0X5F
    }
    #[doc = "Checks if the value of the field is `_0X6F`"]
    #[inline(always)]
    pub fn is_0x6f(&self) -> bool {
        *self == INAC_A::_0X6F
    }
    #[doc = "Checks if the value of the field is `_0X7F`"]
    #[inline(always)]
    pub fn is_0x7f(&self) -> bool {
        *self == INAC_A::_0X7F
    }
    #[doc = "Checks if the value of the field is `_0X8F`"]
    #[inline(always)]
    pub fn is_0x8f(&self) -> bool {
        *self == INAC_A::_0X8F
    }
    #[doc = "Checks if the value of the field is `_0X9F`"]
    #[inline(always)]
    pub fn is_0x9f(&self) -> bool {
        *self == INAC_A::_0X9F
    }
    #[doc = "Checks if the value of the field is `_0X_AF`"]
    #[inline(always)]
    pub fn is_0x_af(&self) -> bool {
        *self == INAC_A::_0X_AF
    }
    #[doc = "Checks if the value of the field is `_0X_BF`"]
    #[inline(always)]
    pub fn is_0x_bf(&self) -> bool {
        *self == INAC_A::_0X_BF
    }
    #[doc = "Checks if the value of the field is `_0X_CF`"]
    #[inline(always)]
    pub fn is_0x_cf(&self) -> bool {
        *self == INAC_A::_0X_CF
    }
    #[doc = "Checks if the value of the field is `_0X_DF`"]
    #[inline(always)]
    pub fn is_0x_df(&self) -> bool {
        *self == INAC_A::_0X_DF
    }
    #[doc = "Checks if the value of the field is `_0X_EF`"]
    #[inline(always)]
    pub fn is_0x_ef(&self) -> bool {
        *self == INAC_A::_0X_EF
    }
    #[doc = "Checks if the value of the field is `_0X_FF`"]
    #[inline(always)]
    pub fn is_0x_ff(&self) -> bool {
        *self == INAC_A::_0X_FF
    }
}
#[doc = "Field `INAC` writer - Inaccuracy Byte"]
pub type INAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CETSM_SPEC, u8, INAC_A, 8, O>;
impl<'a, const O: u8> INAC_W<'a, O> {
    #[doc = "0.0%"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(INAC_A::_0X00)
    }
    #[doc = "1.5%"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(INAC_A::_0X0F)
    }
    #[doc = "3.1%"]
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut W {
        self.variant(INAC_A::_0X1F)
    }
    #[doc = "4.7%"]
    #[inline(always)]
    pub fn _0x2f(self) -> &'a mut W {
        self.variant(INAC_A::_0X2F)
    }
    #[doc = "6.3%"]
    #[inline(always)]
    pub fn _0x3f(self) -> &'a mut W {
        self.variant(INAC_A::_0X3F)
    }
    #[doc = "7.9%"]
    #[inline(always)]
    pub fn _0x4f(self) -> &'a mut W {
        self.variant(INAC_A::_0X4F)
    }
    #[doc = "9.5%"]
    #[inline(always)]
    pub fn _0x5f(self) -> &'a mut W {
        self.variant(INAC_A::_0X5F)
    }
    #[doc = "11.1%"]
    #[inline(always)]
    pub fn _0x6f(self) -> &'a mut W {
        self.variant(INAC_A::_0X6F)
    }
    #[doc = "12.7%"]
    #[inline(always)]
    pub fn _0x7f(self) -> &'a mut W {
        self.variant(INAC_A::_0X7F)
    }
    #[doc = "14.3%"]
    #[inline(always)]
    pub fn _0x8f(self) -> &'a mut W {
        self.variant(INAC_A::_0X8F)
    }
    #[doc = "15.9%"]
    #[inline(always)]
    pub fn _0x9f(self) -> &'a mut W {
        self.variant(INAC_A::_0X9F)
    }
    #[doc = "17.5%"]
    #[inline(always)]
    pub fn _0x_af(self) -> &'a mut W {
        self.variant(INAC_A::_0X_AF)
    }
    #[doc = "19.1%"]
    #[inline(always)]
    pub fn _0x_bf(self) -> &'a mut W {
        self.variant(INAC_A::_0X_BF)
    }
    #[doc = "20.7%"]
    #[inline(always)]
    pub fn _0x_cf(self) -> &'a mut W {
        self.variant(INAC_A::_0X_CF)
    }
    #[doc = "22.3%"]
    #[inline(always)]
    pub fn _0x_df(self) -> &'a mut W {
        self.variant(INAC_A::_0X_DF)
    }
    #[doc = "23.9%"]
    #[inline(always)]
    pub fn _0x_ef(self) -> &'a mut W {
        self.variant(INAC_A::_0X_EF)
    }
    #[doc = "25.5%"]
    #[inline(always)]
    pub fn _0x_ff(self) -> &'a mut W {
        self.variant(INAC_A::_0X_FF)
    }
}
impl R {
    #[doc = "Bits 8:15 - Frequency Byte"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Inaccuracy Byte"]
    #[inline(always)]
    pub fn inac(&self) -> INAC_R {
        INAC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Frequency Byte"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<8> {
        FREQ_W::new(self)
    }
    #[doc = "Bits 16:23 - Inaccuracy Byte"]
    #[inline(always)]
    #[must_use]
    pub fn inac(&mut self) -> INAC_W<16> {
        INAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCC Exchange Timing Support Information M (Mode) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cetsm](index.html) module"]
pub struct CETSM_SPEC;
impl crate::RegisterSpec for CETSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cetsm::R](R) reader structure"]
impl crate::Readable for CETSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cetsm::W](W) writer structure"]
impl crate::Writable for CETSM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CETSM to value 0"]
impl crate::Resettable for CETSM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
